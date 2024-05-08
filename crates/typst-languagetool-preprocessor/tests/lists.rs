mod common;

use typst::syntax::{Source, SyntaxKind};
use typst_languagetool_preprocessor::preprocess;

use crate::common::{node, paragraphs_eq};

#[test]
fn lists() {
    let contents = include_str!("./resources/lists.typ");
    let source = Source::detached(contents);
    let paragraphs = preprocess(&source);

    let expected = vec![
        vec![node(SyntaxKind::Text, "Lorem ipsum dolor sit amet")],
        vec![node(SyntaxKind::Text, "consectetur adipiscing elit")],
        vec![node(SyntaxKind::Text, "sed do eiusmod tempor")],
        vec![
            node(
                SyntaxKind::Text,
                "incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis",
            ),
            node(SyntaxKind::Space, "\r\n  "),
            node(
                SyntaxKind::Text,
                "nostrud exercitation ullamco laboris nisi ut aliquip",
            ),
        ],
        vec![node(SyntaxKind::Text, "ex ea commodo consequat.")],
    ];

    assert!(
        paragraphs_eq(&paragraphs, &expected),
        "Results:\n{:#?}\nExpected:\n{:#?}",
        paragraphs,
        expected
    );
}
