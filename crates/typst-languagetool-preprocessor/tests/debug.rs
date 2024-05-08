use typst::syntax::Source;
use typst_languagetool_preprocessor::preprocess;

#[test]
fn debug() {
    let source = Source::detached(r#""#);

    let paragrapghs = preprocess(&source);
    dbg!(paragrapghs);
}
