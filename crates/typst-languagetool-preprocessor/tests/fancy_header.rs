mod common;

use typst::syntax::{Source, SyntaxKind};
use typst_languagetool_preprocessor::preprocess;

use crate::common::{node, paragraphs_eq};

#[test]
fn fancy_header() {
    let contents = include_str!("./resources/fancy_header.typ");
    let source = Source::detached(contents);
    let paragraphs = preprocess(&source);

    let expected = vec![
        vec![
            node(SyntaxKind::Text, "Trent Shailer"),
            node(SyntaxKind::Space, "\r\n"),
        ],
        vec![
            node(SyntaxKind::Text, "2024"),
            node(SyntaxKind::Space, "\r\n"),
        ],
    ];

    assert!(
        paragraphs_eq(&paragraphs, &expected),
        "Results:\n{:#?}\nExpected:\n{:#?}",
        paragraphs,
        expected
    );
}
