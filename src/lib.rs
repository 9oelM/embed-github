use errors::{
    ConvertGithubPermalinkToRawGithubUserContentError, DecodeLineRangeError, GetSourceError,
    HighlightCodeError, OptionsBuilderError, RequestError, RequestedSourceInfoError,
};
use syntect::{
    highlighting::{Color, ThemeSet},
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};
use worker::*;

mod errors;
mod hydration;
mod utils;

const PERMALINK_MIN_PATH_LEN: u32 = 5;
const DEFAULT_THEME: &str = "base16-ocean.dark";
const EMBED_GITHUB_URL: &str = "https://github.com/9oelm/embed-github";
const STYLE: &str = include_str!("style.css");

#[derive(Debug)]
enum LineRange {
    Single(u64),
    Range(u64, u64),
    All,
}

#[derive(Debug)]
struct RawGithubUserContentSource {
    raw_code_url: Url,
    file_extension: String,
}

#[derive(Debug)]
struct RequestedSourceInfo {
    url: Url,
    lines: LineRange,
}

#[derive(Debug)]
struct OptionsBuilder {
    gh: String,
    theme: String,
    lines: Option<String>,
    lang: Option<String>,
}

#[derive(Debug)]
struct Options {
    requested_source_info: RequestedSourceInfo,
    raw_github_user_content: RawGithubUserContentSource,
    theme: String,
    lang: Option<String>,
}

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    let url = match req.url() {
        Ok(url) => url,
        Err(_) => {
            let req = RequestError::ReqToUrl {
                method: req.method(),
                path: req.path(),
            };

            return req.into();
        }
    };

    if url.query().is_none() {
        return RequestError::EmptyQuerystring.into();
    }
    let options_builder = OptionsBuilder::from_url(&url);

    let options_builder = match options_builder {
        Ok(options_builder) => options_builder,
        Err(err) => return err.into(),
    };

    let options = match options_builder.build().await {
        Ok(options) => options,
        Err(err) => return err.into(),
    };

    let source_code_in_range = match get_source_code_in_range(
        options.raw_github_user_content.raw_code_url.clone(),
        &options.requested_source_info.lines,
    )
    .await
    {
        Ok(source_code_in_range) => source_code_in_range,
        Err(err) => return err.into(),
    };

    let highlighted_code = match highlight_code(&source_code_in_range, &options) {
        Ok(highlighted_code) => highlighted_code,
        Err(err) => return err.into(),
    };

    Response::from_html(highlighted_code)
}

// https://github.com/huggingface/transformers/blob/0fdea8607d7e01eb0e38a1ebeb7feee30a22f0cf/tests/models/align/test_modeling_align.py#L10-L19 =>
// https://raw.githubusercontent.com/huggingface/transformers/0fdea8607d7e01eb0e38a1ebeb7feee30a22f0cf/tests/models/align/test_modeling_align.py
fn convert_github_permalink_to_raw_githubusercontent_source(
    github_permalink: Url,
) -> core::result::Result<
    RawGithubUserContentSource,
    ConvertGithubPermalinkToRawGithubUserContentError,
> {
    let path = github_permalink.path().split('/').collect::<Vec<&str>>();
    if path.len() < PERMALINK_MIN_PATH_LEN as usize {
        return Err(
            ConvertGithubPermalinkToRawGithubUserContentError::PathTooShort(
                github_permalink.as_str().to_string(),
            ),
        );
    }

    // Path length already checked. First path is empty.
    let user_slash_repository = path[1..3].join("/");
    // Omit 'blob' at path[3]
    let hash = path[4];
    let file_path = path[5..].join("/");

    // If file extension is not found, default to 'txt'
    let file_extension = path[path.len() - 1].split('.').last().unwrap_or("txt");

    let raw_code_url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}",
        user_slash_repository, hash, file_path
    );
    let raw_code_url = Url::parse(&raw_code_url).map_err(|_| {
        ConvertGithubPermalinkToRawGithubUserContentError::ComposeRawCodeUrl { url: raw_code_url }
    })?;

    Ok(RawGithubUserContentSource {
        raw_code_url,
        file_extension: file_extension.to_string(),
    })
}

fn highlight_code(
    source_code_in_range: &str,
    options: &Options,
) -> core::result::Result<String, HighlightCodeError> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // Priority: lang query parameter > source file extension > plaintext
    let sr = ss
        .find_syntax_by_token(
            &options
                .lang
                .clone()
                .unwrap_or(options.raw_github_user_content.file_extension.clone()),
        )
        .or_else(|| {
            // default to plaintext if no matching syntax is found
            ss.find_syntax_by_extension("txt")
        })
        .unwrap();

    let mut html = String::from("");

    // raw_code_url must contain at least one path segment
    let file_path = options
        .raw_github_user_content
        .raw_code_url
        .path()
        .split('/')
        .last()
        .unwrap();
    html += &format!(
        "<head><title>{}</title><style>{}</style><script>{}</script></head>",
        file_path,
        STYLE,
        hydration::generate_hydration_script(&options.requested_source_info.lines)
    );
    let theme = &ts
        .themes
        .get(&options.theme)
        .ok_or(HighlightCodeError::InvalidTheme(options.theme.clone()))?;
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    html += &format!(
        "<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
        c.r, c.g, c.b
    );
    html += "<main><aside class=\"line-numbers\"></aside>";
    let highlighted_code =
        highlighted_html_for_string(source_code_in_range, &ss, sr, theme).unwrap();
    html += &highlighted_code.to_string();

    let original_url_with_line_range = match options.requested_source_info.lines {
        LineRange::Single(line) => {
            format!("{}#L{}", options.requested_source_info.url.as_str(), line)
        }
        LineRange::Range(start, end) => {
            format!(
                "{}#L{}-L{}",
                options.requested_source_info.url.as_str(),
                start,
                end
            )
        }
        LineRange::All => options.requested_source_info.url.as_str().to_string(),
    };
    let line_info = match options.requested_source_info.lines.to_string().as_str() {
        "" => "".to_string(),
        s => format!("#{}", s),
    };

    html += &format!(
        "<div class=\"bottom-bar\">
        <a href=\"{}\" class=\"link\" target=\"_blank\">{}{}</a>
        <a href=\"{}\" class=\"link embed-github\" target=\"_blank\">hosted by 9oelm/embed-github</a>
    </div></main></body>",
        &original_url_with_line_range, file_path, line_info, EMBED_GITHUB_URL
    );

    Ok(html)
}

fn decode_line_range(fragment: &str) -> core::result::Result<LineRange, DecodeLineRangeError> {
    let line_numbers = fragment.split('-').collect::<Vec<&str>>();

    if line_numbers.len() == 2 {
        let start: u64 = line_numbers[0]
            .strip_prefix("L")
            .ok_or(DecodeLineRangeError::ParseLineNumber {
                fragment: fragment.to_string(),
            })?
            .parse()
            .map_err(|_| DecodeLineRangeError::ParseLineNumber {
                fragment: fragment.to_string(),
            })?;
        let end: u64 = line_numbers[1]
            .strip_prefix("L")
            .ok_or(DecodeLineRangeError::ParseLineNumber {
                fragment: fragment.to_string(),
            })?
            .parse()
            .map_err(|_| DecodeLineRangeError::ParseLineNumber {
                fragment: fragment.to_string(),
            })?;

        if start > end {
            return Err(DecodeLineRangeError::LineStartBiggerThanEnd { start, end });
        }

        Ok(LineRange::Range(start, end))
    } else if line_numbers.len() == 1 {
        let single_line: u64 = line_numbers[0]
            .strip_prefix("L")
            .ok_or(DecodeLineRangeError::ParseLineNumber {
                fragment: fragment.to_string(),
            })?
            .parse()
            .map_err(|_| DecodeLineRangeError::ParseLineNumber {
                fragment: fragment.to_string(),
            })?;
        Ok(LineRange::Single(single_line))
    } else {
        Err(DecodeLineRangeError::ParseLineNumber {
            fragment: fragment.to_string(),
        })
    }
}

async fn get(url: Url) -> core::result::Result<String, GetSourceError> {
    let mut resp = Fetch::Url(url)
        .send()
        .await
        .map_err(GetSourceError::FetchSource)?;
    let txt = resp.text().await.map_err(GetSourceError::FetchSource)?;

    Ok(txt)
}

async fn get_source_code_in_range(
    raw_code_url: Url,
    line_range: &LineRange,
) -> core::result::Result<String, GetSourceError> {
    match &line_range {
        LineRange::Single(line) => {
            let source_code = get(raw_code_url).await?;
            let in_range = source_code
                .lines()
                .nth(*line as usize - 1)
                .map(|line| line.to_string())
                .ok_or(GetSourceError::LineNumberOutOfRange { number: *line })?;

            Ok(in_range)
        }
        LineRange::Range(start, end) => {
            let source_code = get(raw_code_url).await?;
            let in_range = source_code
                .lines()
                .skip(*start as usize - 1)
                .take(*end as usize - *start as usize + 1)
                .collect::<Vec<&str>>()
                .join("\n");

            Ok(in_range)
        }
        LineRange::All => get(raw_code_url).await,
    }
}

fn get_requested_source_info_from_query(
    gh_query_param: &str,
    lines_query_param: &Option<String>,
) -> core::result::Result<RequestedSourceInfo, RequestedSourceInfoError> {
    let url = Url::parse(gh_query_param)
        .map_err(|_| RequestedSourceInfoError::InvalidGhQueryUrl(gh_query_param.to_string()))?;
    let domain = url.domain().ok_or(RequestedSourceInfoError::ParseDomain(
        gh_query_param.to_string(),
    ))?;

    if domain != "github.com" {
        return Err(RequestedSourceInfoError::DomainNotGithub(
            gh_query_param.to_string(),
        ));
    }

    let line_range_from_fragment = if let Some(line_numbers) = url.fragment() {
        decode_line_range(line_numbers).map_err(RequestedSourceInfoError::DecodeLineRange)?
    } else {
        LineRange::All
    };

    // Priority: lines query parameter > line range from fragment
    let line_range = if let Some(line_numbers) = lines_query_param {
        decode_line_range(line_numbers).map_err(RequestedSourceInfoError::DecodeLineRange)?
    } else {
        line_range_from_fragment
    };

    Ok(RequestedSourceInfo {
        url,
        lines: line_range,
    })
}

impl OptionsBuilder {
    fn from_url(url: &Url) -> core::result::Result<OptionsBuilder, OptionsBuilderError> {
        let mut options_builder = OptionsBuilder::default();
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "gh" => options_builder.gh = value.to_string(),
                "theme" => options_builder.theme = value.to_string(),
                "lines" => options_builder.lines = Some(value.to_string()),
                "lang" => options_builder.lang = Some(value.to_string()),
                _ => {}
            }
        }

        if options_builder.gh.is_empty() {
            return Err(OptionsBuilderError::MissingGhQuery);
        }

        options_builder.theme = if options_builder.theme.is_empty() {
            DEFAULT_THEME.to_string()
        } else {
            options_builder.theme
        };

        Ok(OptionsBuilder {
            gh: options_builder.gh,
            theme: options_builder.theme,
            lines: options_builder.lines,
            lang: options_builder.lang,
        })
    }

    async fn build(self) -> core::result::Result<Options, OptionsBuilderError> {
        let requested_source_info = get_requested_source_info_from_query(&self.gh, &self.lines)
            .map_err(OptionsBuilderError::RequestedSourceInfo)?;

        let raw_github_user_content = convert_github_permalink_to_raw_githubusercontent_source(
            requested_source_info.url.clone(),
        )
        .map_err(OptionsBuilderError::ConvertGithubPermalinkToRawGithubUserContent)?;

        Ok(Options {
            requested_source_info,
            raw_github_user_content,
            theme: self.theme,
            lang: self.lang,
        })
    }
}

impl Default for OptionsBuilder {
    fn default() -> Self {
        OptionsBuilder {
            gh: "".to_string(),
            theme: DEFAULT_THEME.to_string(),
            lines: None,
            lang: None,
        }
    }
}

impl std::fmt::Display for LineRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineRange::Single(line) => write!(f, "L{}", line),
            LineRange::Range(start, end) => write!(f, "L{}-L{}", start, end),
            LineRange::All => write!(f, ""),
        }
    }
}
