#![deny(clippy::all)]
use tree_sitter_highlight::Highlighter;
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::HighlightEvent;
use html_escape::encode_safe;

#[macro_use]
extern crate napi_derive;


const NAMES: &[&str] = &[
    "attribute",
    "carriage-return",
    "comment",
    "comment.documentation",
    "constant",
    "constant.builtin",
    "constructor",
    "constructor.builtin",
    "embedded",
    "error",
    "escape",
    "function",
    "function.builtin",
    "include",
    "keyword",
    "markup",
    "markup.bold",
    "markup.heading",
    "markup.italic",
    "markup.link",
    "markup.link.url",
    "markup.list",
    "markup.list.checked",
    "markup.list.numbered",
    "markup.list.unchecked",
    "markup.list.unnumbered",
    "markup.quote",
    "markup.raw",
    "markup.raw.block",
    "markup.raw.inline",
    "markup.strikethrough",
    "module",
    "number",
    "operator",
    "property",
    "property.builtin",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regexp",
    "string.special",
    "string.special.symbol",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
];


fn get_config(name: &str) -> Option<HighlightConfiguration> {
    match name {
        "css" | "scss" => HighlightConfiguration::new(
            tree_sitter_css::language(),
            tree_sitter_css::HIGHLIGHTS_QUERY,
            "",
            "",
        ).ok(),
        "haskell" => HighlightConfiguration::new(
            tree_sitter_haskell::language(),
            tree_sitter_haskell::HIGHLIGHTS_QUERY,
            "",
            tree_sitter_haskell::LOCALS_QUERY,
        ).ok(),
        "rust" => HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            ""
        ).ok(),
        _ => None
    }
}

#[napi]
pub fn hl(lang: String, src: String) -> String {
    let mut config = match get_config(&lang) {
        Some(c) => c,
        None => return encode_safe(&src).to_string()
    };

    config.configure(NAMES);

    let mut highlighter = Highlighter::new();
    let highlights = highlighter.highlight(
        &config,
        src.as_bytes(),
        None,
        |_| None
    ).unwrap();

    let mut out = String::new();
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source {start, end} => {
                out.push_str(&encode_safe(&src[start..end]));
            },
            HighlightEvent::HighlightStart(s) => {
                out.push_str("<span class=\"");
                out.push_str(NAMES[s.0]);
                out.push_str("\">");
            },
            HighlightEvent::HighlightEnd => {
                out.push_str("</span>");
            },
        }
    }

    out
}

