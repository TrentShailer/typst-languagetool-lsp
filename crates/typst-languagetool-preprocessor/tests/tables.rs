mod common;

use typst::syntax::{Source, SyntaxKind};
use typst_languagetool_preprocessor::preprocess;

use crate::common::{node, paragraphs_eq};

#[test]
fn tables() {
    let contents = include_str!("./resources/tables.typ");
    let source = Source::detached(contents);
    let paragraphs = preprocess(&source);

    let expected = vec![
        vec![node(SyntaxKind::Text, "Heading 1")],
        vec![node(SyntaxKind::Text, "Heading 2")],
        vec![node(SyntaxKind::Text, "Heading 3")],
        vec![node(SyntaxKind::Text, "Lorem ipsum dolor sit amet")],
        vec![node(SyntaxKind::Text, "Lorem ipsum dolor sit amet")],
        vec![
            node(
                SyntaxKind::Text,
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor",
            ),
            node(SyntaxKind::Space, "\r\n    "),
            node(
                SyntaxKind::Text,
                "incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis",
            ),
            node(SyntaxKind::Space, "\r\n    "),
            node(
                SyntaxKind::Text,
                "nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
            ),
        ],
    ];

    assert!(
        paragraphs_eq(&paragraphs, &expected),
        "Results:\n{:#?}\nExpected:\n{:#?}",
        paragraphs,
        expected
    );
}
