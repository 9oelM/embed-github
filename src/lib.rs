use std::fmt;

use syntect::{highlighting::{Color, ThemeSet}, html::{highlighted_html_for_file, highlighted_html_for_string}, parsing::{SyntaxReference, SyntaxSet}};
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let highlighted_code = highlight_code();

    Response::from_html(highlighted_code)
}

fn highlight_code() -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let sr = ss.syntaxes();

    console_log!("{:?}", sr);

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
    let mut result = format!(
        "<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
        c.r, c.g, c.b
    );
    let highlighted_code = "";
    // let html = highlighted_html_for_string("const a = 1;", &ss, sr, theme).unwrap();
    html += &format!("{}", highlighted_code);
    html += &format!("</body>");

    html
}