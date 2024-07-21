use syntect::{
    highlighting::{Color, ThemeSet},
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};
use worker::*;

mod hydration;
mod utils;

const PERMALINK_MIN_PATH_LEN: u32 = 5;
const DEFAULT_THEME: &str = "base16-ocean.dark";
const EMBED_GITHUB_URL: &str = "https://github.com/9oelm/embed-github";

enum Conversion {
    PathTooShort,
    ParseLineNumber,
    InvalidLinePrefix,
    InvalidLineNumberFormat,
    InvalidLineNumberPair,
    ComposeRawCodeUrl,
    LineNumberOutOfRange,
}

enum HighlightCodeError {
    InvalidTheme,
}

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

    let url = req.url()?;

    // If query parameter is not provided (for any other requests, like GET /favicon.ico), return an instruction.
    if url.query().is_none() {
        return Response::from_html("<html>See <a href=\"https://github.com/9oelM/embed-github\">https://github.com/9oelM/embed-github</a> on how to use this.</html>");
    }
    let options_builder = OptionsBuilder::from_url(&url)?;
    let options = options_builder.build().await?;

    let source_code_in_range = get_source_code_in_range(
        options.raw_github_user_content.raw_code_url.clone(),
        &options.requested_source_info.lines,
    )
    .await?;

    let highlighted_code = highlight_code(&source_code_in_range, &options)?;

    Response::from_html(highlighted_code)
}

// https://github.com/huggingface/transformers/blob/0fdea8607d7e01eb0e38a1ebeb7feee30a22f0cf/tests/models/align/test_modeling_align.py#L10-L19 =>
// https://raw.githubusercontent.com/huggingface/transformers/0fdea8607d7e01eb0e38a1ebeb7feee30a22f0cf/tests/models/align/test_modeling_align.py
async fn convert_github_permalink_to_raw_githubusercontent_source(
    github_permalink: Url,
) -> Result<RawGithubUserContentSource> {
    let path = github_permalink.path().split('/').collect::<Vec<&str>>();
    if path.len() < PERMALINK_MIN_PATH_LEN as usize {
        return Err(Conversion::PathTooShort.into());
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
    let raw_code_url = Url::parse(&raw_code_url).map_err(|_| Conversion::ComposeRawCodeUrl)?;

    Ok(RawGithubUserContentSource {
        raw_code_url,
        file_extension: file_extension.to_string(),
    })
}

fn highlight_code(source_code_in_range: &str, options: &Options) -> Result<String> {
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
    let style = "
        pre {
            font-size: 13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
            width: calc(100% - 50px);
        }
        html, body, p, pre {
            margin: 0;
            padding: 0;
        }
        main {
            display: flex;
            flex-direction: row;
            margin-top: 10px;
            margin-bottom: 100px;
            column-gap: 10px;
        }
        .bottom-bar {
            position: fixed;
            bottom: 0;
            left: 0;
            width: 100%;
            background-color: #f8f8f8;
            border-top: 1px solid #e0e0e0;
            font-size: 0.8em;
            display: flex;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }
        .link {
            color: #0074d9;
            text-decoration: none;
            transition: color 0.2s;
            padding: 0.5em;
            word-break: break-all;
        }
        .link:hover {
            color: #012fff;
            text-decoration: underline;
        }
        .embed-github {
            margin-left: auto;
        }
        .line-numbers {
            width: 40px;
            height: 100%;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
            color: #fff;
            font-size: 13px;
        }
    ";
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
        style,
        hydration::generate_hydration_script(&options.requested_source_info.lines)
    );
    let theme = &ts
        .themes
        .get(&options.theme)
        .ok_or::<Error>(HighlightCodeError::InvalidTheme.into())?;
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

fn decode_line_range(fragment: &str) -> Result<LineRange> {
    let line_numbers = fragment.split('-').collect::<Vec<&str>>();

    if line_numbers.len() == 2 {
        let start: u64 = line_numbers[0]
            .strip_prefix("L")
            .ok_or(Conversion::InvalidLinePrefix)?
            .parse()
            .map_err(|_| Conversion::ParseLineNumber)?;
        let end: u64 = line_numbers[1]
            .strip_prefix("L")
            .ok_or(Conversion::InvalidLinePrefix)?
            .parse()
            .map_err(|_| Conversion::ParseLineNumber)?;

        if start > end {
            return Err(Conversion::InvalidLineNumberPair.into());
        }

        Ok(LineRange::Range(start, end))
    } else if line_numbers.len() == 1 {
        let single_line: u64 = line_numbers[0]
            .strip_prefix("L")
            .ok_or(Conversion::InvalidLinePrefix)?
            .parse()
            .map_err(|_| Conversion::ParseLineNumber)?;
        Ok(LineRange::Single(single_line))
    } else {
        Err(Conversion::InvalidLineNumberFormat.into())
    }
}

async fn get(url: Url) -> Result<String> {
    let mut resp = Fetch::Url(url).send().await?;
    let txt = resp.text().await?;

    Ok(txt)
}

async fn get_source_code_in_range(raw_code_url: Url, line_range: &LineRange) -> Result<String> {
    match &line_range {
        LineRange::Single(line) => {
            let source_code = get(raw_code_url).await?;
            let in_range = source_code
                .lines()
                .nth(*line as usize - 1)
                .map(|line| line.to_string())
                .ok_or(Conversion::LineNumberOutOfRange)?;

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
) -> Result<RequestedSourceInfo> {
    let url = Url::parse(gh_query_param).map_err(|_| {
        Error::RustError("Invalid Github Permalink URL as a query parameter".to_string())
    })?;
    let domain = url.domain().ok_or(Error::RustError(
        "Invalid Github Permalink URL missing a domain".to_string(),
    ))?;

    if domain != "github.com" {
        return Err(Error::RustError(
            "Invalid Github Permalink URL. Only URLs from github.com are allowed".to_string(),
        ));
    }

    let line_range_from_fragment = if let Some(line_numbers) = url.fragment() {
        decode_line_range(line_numbers)?
    } else {
        LineRange::All
    };

    // Priority: lines query parameter > line range from fragment
    let line_range = if let Some(line_numbers) = lines_query_param {
        decode_line_range(line_numbers)?
    } else {
        line_range_from_fragment
    };

    Ok(RequestedSourceInfo {
        url,
        lines: line_range,
    })
}

impl OptionsBuilder {
    fn from_url(url: &Url) -> Result<OptionsBuilder> {
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
            return Err(Error::RustError("\"gh\" parameter is required. See https://github.com/9oelM/embed-github on how to use this".to_string()));
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

    async fn build(self) -> Result<Options> {
        let requested_source_info = get_requested_source_info_from_query(&self.gh, &self.lines)?;

        let raw_github_user_content = convert_github_permalink_to_raw_githubusercontent_source(
            requested_source_info.url.clone(),
        )
        .await?;

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

impl From<Conversion> for Error {
    fn from(err: Conversion) -> Self {
        match err {
            Conversion::PathTooShort => Error::RustError(format!(
                "Provided URL's path is too short. Minimum length of path is {}.",
                PERMALINK_MIN_PATH_LEN
            )),
            Conversion::ParseLineNumber => {
                Error::RustError("Proivided line number cannot be converted to u64.".to_string())
            }
            Conversion::InvalidLinePrefix => {
                Error::RustError("Could not parse prefix 'L' of line number.".to_string())
            }
            Conversion::InvalidLineNumberFormat => {
                Error::RustError("Line number format is not #L{number}-L{number}".to_string())
            }
            Conversion::InvalidLineNumberPair => Error::RustError(
                "The first line number must be smaller than the next line number".to_string(),
            ),
            Conversion::ComposeRawCodeUrl => {
                Error::RustError("Could not compose raw code URL.".to_string())
            }
            Conversion::LineNumberOutOfRange => Error::RustError(
                "Corresponding line number could not be found in the file".to_string(),
            ),
        }
    }
}

impl From<HighlightCodeError> for Error {
    fn from(err: HighlightCodeError) -> Self {
        match err {
            HighlightCodeError::InvalidTheme => Error::RustError("Invalid theme".to_string()),
        }
    }
}

impl ToString for LineRange {
    fn to_string(&self) -> String {
        match self {
            LineRange::Single(line) => format!("L{}", line),
            LineRange::Range(start, end) => format!("L{}-L{}", start, end),
            LineRange::All => "".to_string(),
        }
    }
}
