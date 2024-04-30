use typst::syntax::{Source, SyntaxNode};

mod paragraph;

use paragraph::get_paragraphs;

pub fn preprocess(source: &Source) -> Vec<Vec<&SyntaxNode>> {
    let paragraphs = get_paragraphs(source.root());

    paragraphs
}
