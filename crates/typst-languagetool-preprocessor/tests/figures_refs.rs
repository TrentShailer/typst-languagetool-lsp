mod common;

use typst::syntax::{Source, SyntaxKind};
use typst_languagetool_preprocessor::preprocess;

use crate::common::{node, paragraphs_eq};

#[test]
fn figures_refs() {
    let contents = include_str!("./resources/figures_refs.typ");
    let source = Source::detached(contents);
    let paragraphs = preprocess(&source);

    let expected = vec![
        vec![
            node(SyntaxKind::Text, "Lorem ipsum dolor sit amet"),
            node(SyntaxKind::Space, " "),
            node(SyntaxKind::RefMarker, "@test"),
            node(SyntaxKind::Text, "."),
        ],
        vec![node(SyntaxKind::Text, "Caption")],
    ];

    assert!(
        paragraphs_eq(&paragraphs, &expected),
        "Results:\n{:#?}\nExpected:\n{:#?}",
        paragraphs,
        expected
    );
}
