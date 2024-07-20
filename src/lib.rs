use syntect::{
    highlighting::{Color, ThemeSet},
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};
use worker::*;

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

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    // If query parameter is not provided (for any other requests, like GET /favicon.ico), return empty response.
    if url.query().is_none() {
        return Response::from_html("");
    }

    let requested_source_info = get_requested_source_info_from_query(&url)?;
    let permalink_raw_source_code =
        convert_github_permalink_to_raw_githubusercontent_source(requested_source_info.url.clone())
            .await?;
    let source_code_in_range = get_source_code_in_range(
        permalink_raw_source_code.raw_code_url.clone(),
        &requested_source_info.lines,
    )
    .await?;

    let requested_theme = url
        .query_pairs()
        .find(|(key, _)| key == "theme")
        .map(|(_, value)| value);
    let requested_theme = if let Some(theme) = requested_theme {
        &theme.to_string()
    } else {
        DEFAULT_THEME
    };

    let highlighted_code = highlight_code(
        &permalink_raw_source_code,
        &source_code_in_range,
        requested_theme,
        &requested_source_info,
    )?;

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

fn highlight_code(
    permalink_raw_source_code: &RawGithubUserContentSource,
    source_code_in_range: &str,
    requested_theme: &str,
    requested_source_info: &RequestedSourceInfo,
) -> Result<String> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let sr = ss
        .find_syntax_by_extension(&permalink_raw_source_code.file_extension)
        .or_else(|| {
            // default to plaintext if no matching syntax is found
            ss.find_syntax_by_extension("txt")
        })
        .unwrap();

    let mut html = String::from("");
    let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }
        html, body {
            margin: 0;
            padding: 0;
        }
        body {
            padding: 1em;
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
    ";
    // raw_code_url must contain at least one path segment
    let file_path = permalink_raw_source_code
        .raw_code_url
        .path()
        .split('/')
        .last()
        .unwrap();
    html += &format!(
        "<head><title>{}</title><style>{}</style></head>",
        file_path, style
    );
    let theme = &ts
        .themes
        .get(requested_theme)
        .ok_or::<Error>(HighlightCodeError::InvalidTheme.into())?;
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    html += &format!(
        "<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
        c.r, c.g, c.b
    );
    let highlighted_code =
        highlighted_html_for_string(source_code_in_range, &ss, sr, theme).unwrap();
    html += &highlighted_code.to_string();

    let original_url = match requested_source_info.lines {
        LineRange::Single(line) => format!("{}#L{}", requested_source_info.url.as_str(), line),
        LineRange::Range(start, end) => {
            format!("{}#L{}-L{}", requested_source_info.url.as_str(), start, end)
        }
        LineRange::All => requested_source_info.url.as_str().to_string(),
    };
    let line_info = match requested_source_info.lines.to_string().as_str() {
        "" => "".to_string(),
        s => format!("#{}", s),
    };

    html += &format!(
        "<div class=\"bottom-bar\">
        <a href=\"{}\" class=\"link\">{}{}</a>
        <a href=\"{}\" class=\"link embed-github\">hosted by 9oelm/embed-github</a>
    </div></body>",
        &original_url, file_path, line_info, EMBED_GITHUB_URL
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

fn get_requested_source_info_from_query(url: &Url) -> Result<RequestedSourceInfo> {
    let mut query_pairs = url.query_pairs();
    let github_permalink_query_pair = query_pairs
        .find(|(key, _)| key == "gh")
        .ok_or(Error::RustError(
            "\"gh\" parameter is required. Example: https://worker.url?gh=<encoded GitHub permalink URL>"
                .to_string(),
        ))?;

    let decoded_url = js_sys::decode_uri_component(&github_permalink_query_pair.1)?.as_string();

    if decoded_url.clone().is_some_and(
        // Normal URL when decoded returns the same URL
        |decoded_url| decoded_url != github_permalink_query_pair.1,
    ) {
        // Already checked
        let decoded_url = decoded_url.unwrap();
        let url = Url::parse(&decoded_url).map_err(|_| {
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

        let line_range = if let Some(frag) = url.fragment() {
            decode_line_range(frag)?
        } else {
            LineRange::All
        };

        Ok(RequestedSourceInfo {
            url,
            lines: line_range,
        })
    } else {
        let line_numbers = url.query_pairs().clone().find(|(key, _)| key == "lines");

        let url = Url::parse(&github_permalink_query_pair.1).map_err(|_| {
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

        let line_range = if let Some(line_numbers) = line_numbers {
            decode_line_range(&line_numbers.1)?
        } else {
            LineRange::All
        };

        Ok(RequestedSourceInfo {
            url,
            lines: line_range,
        })
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
