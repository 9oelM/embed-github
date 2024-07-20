use syntect::{
    highlighting::{Color, ThemeSet},
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};
use worker::*;

const PERMALINK_MIN_PATH_LEN: u32 = 5;

enum Conversion {
    PathTooShort,
    ParseLineNumber,
    InvalidLinePrefix,
    InvalidLineNumberFormat,
    InvalidLineNumberPair,
    ComposeRawCodeUrl,
    LineNumberOutOfRange,
}

#[derive(Debug)]
enum LineRange {
    Single(u64),
    Range(u64, u64),
    All,
}

#[derive(Debug)]
struct PermalinkRawSourceCode {
    raw_code_url: Url,
    file_extension: String,
    lines: LineRange,
}

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let permalink_raw_source_code = convert_github_permalink_to_permalink_source(Url::parse("https://github.com/huggingface/transformers/blob/0fdea8607d7e01eb0e38a1ebeb7feee30a22f0cf/tests/models/align/test_modeling_align.py#L10").unwrap()).await?;
    let source_code_in_range = get_source_code_in_range(
        permalink_raw_source_code.raw_code_url.clone(),
        &permalink_raw_source_code.lines,
    )
    .await?;

    console_log!("{}", source_code_in_range);

    let highlighted_code = highlight_code(&permalink_raw_source_code, &source_code_in_range);

    Response::from_html(highlighted_code)
}

// https://github.com/huggingface/transformers/blob/0fdea8607d7e01eb0e38a1ebeb7feee30a22f0cf/tests/models/align/test_modeling_align.py#L10-L19 =>
// https://raw.githubusercontent.com/huggingface/transformers/0fdea8607d7e01eb0e38a1ebeb7feee30a22f0cf/tests/models/align/test_modeling_align.py
async fn convert_github_permalink_to_permalink_source(
    github_permalink: Url,
) -> Result<PermalinkRawSourceCode> {
    let path = github_permalink.path().split('/').collect::<Vec<&str>>();
    if path.len() < PERMALINK_MIN_PATH_LEN as usize {
        return Err(Conversion::PathTooShort.into());
    }

    // Path length already checked. First path is empty.
    let user_slash_repository = path[1..3].join("/");
    // Omit 'blob' at path[3]
    let hash = path[4];
    let file_path = path[5..].join("/");
    // #L10-L19
    let fragment = github_permalink.fragment();

    // If file extension is not found, default to 'txt'
    let file_extension = path[path.len() - 1].split('.').last().unwrap_or("txt");

    let raw_code_url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}",
        user_slash_repository, hash, file_path
    );
    let raw_code_url = Url::parse(&raw_code_url).map_err(|_| Conversion::ComposeRawCodeUrl)?;

    let line_range = if let Some(fragment) = fragment {
        decode_line_range(fragment)?
    } else {
        LineRange::All
    };

    Ok(PermalinkRawSourceCode {
        raw_code_url,
        file_extension: file_extension.to_string(),
        lines: line_range,
    })
}

fn highlight_code(
    permalink_raw_source_code: &PermalinkRawSourceCode,
    source_code_in_range: &str,
) -> String {
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
        }";
    html += &format!("<head><title>test</title><style>{}</style></head>", style);
    let theme = &ts.themes["base16-ocean.dark"];
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    html += &format!(
        "<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
        c.r, c.g, c.b
    );
    let highlighted_code =
        highlighted_html_for_string(&source_code_in_range, &ss, sr, theme).unwrap();
    html += &format!("{}", highlighted_code);
    html += &format!("</body>");

    html
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
