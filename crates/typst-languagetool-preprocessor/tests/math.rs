mod common;

use typst::syntax::Source;
use typst_languagetool_preprocessor::preprocess;

use crate::common::paragraphs_eq;

#[test]
fn maths() {
    let contents = include_str!("./resources/maths.typ");
    let source = Source::detached(contents);
    let paragraphs = preprocess(&source);

    let expected = vec![];

    assert!(
        paragraphs_eq(&paragraphs, &expected),
        "Results:\n{:#?}\nExpected:\n{:#?}",
        paragraphs,
        expected
    );
}
