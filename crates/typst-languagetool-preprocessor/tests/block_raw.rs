mod common;

use typst::syntax::{Source, SyntaxKind};
use typst_languagetool_preprocessor::preprocess;

use crate::common::{node, paragraphs_eq};

#[test]
fn block_raw() {
    let contents = include_str!("./resources/block_raw.typ");
    let source = Source::detached(contents);
    let paragraphs = preprocess(&source);

    let expected = vec![
        vec![node(
            SyntaxKind::Text,
            "Lorem ipsum dolor sit amet, consectetur adipiscing.",
        )],
        vec![
            node(
                SyntaxKind::Text,
                "Lorem ipsum dolor sit amet, consectetur adipiscing.",
            ),
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
