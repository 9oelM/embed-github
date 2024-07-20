use std::fmt;

use syntect::{highlighting::{Color, ThemeSet}, html::{highlighted_html_for_file, highlighted_html_for_string}, parsing::{SyntaxReference, SyntaxSet}};
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let highlighted_code = highlight_code("js");

    Response::from_html(highlighted_code)
}

fn highlight_code(file_extension: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let sr = ss.find_syntax_by_extension(file_extension).or_else(|| {
        // default to plaintext if no matching syntax is found
        ss.find_syntax_by_extension("txt")
    }).unwrap();

    let mut html = String::from("");
    let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }";
    html += &format!(
        "<head><title>test</title><style>{}</style></head>",
        style
    );
    let theme = &ts.themes["base16-ocean.dark"];
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    html += &format!(
        "<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
        c.r, c.g, c.b
    );
    let test = "export const randomAddress = (wc: number = 0) => {
    const buf = Buffer.alloc(32);
    for (let i = 0; i < buf.length; i++) {
        buf[i] = Math.floor(Math.random() * 256);
    }
    return new Address(wc, buf);
};";
    let highlighted_code = highlighted_html_for_string(test, &ss, sr, theme).unwrap();
    html += &format!("{}", highlighted_code);
    html += &format!("</body>");

    html
}