#![deny(clippy::all)]
use tree_sitter_highlight::Highlighter;
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::HighlightEvent;

#[macro_use]
extern crate napi_derive;




#[napi]
pub fn hl(test: String) -> Vec<String> {
    let highlight_names = &[
        "module",
        "attribute",
        "constant",
        "function.builtin",
        "function",
        "keyword",
        "operator",
        "property",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "string",
        "string.special",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.parameter",
    ];


    let mut config = HighlightConfiguration::new(
        tree_sitter_haskell::language(),
        tree_sitter_haskell::HIGHLIGHTS_QUERY,
        "",
        tree_sitter_haskell::LOCALS_QUERY,
    ).unwrap();

    config.configure(highlight_names);

    let mut highlighter = Highlighter::new();
    let highlights = highlighter.highlight(
        &config,
        test.as_bytes(),
        None,
        |_| None
    ).unwrap();

    let mut evs = vec![];
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source {start, end} => {
                evs.push(format!("source: {start}-{end}"));
            },
            HighlightEvent::HighlightStart(s) => {
                evs.push(format!("highlight style started: {:?}", s));
            },
            HighlightEvent::HighlightEnd => {
                evs.push(format!("highlight style ended"));
            },
        }
    }

    evs
}

