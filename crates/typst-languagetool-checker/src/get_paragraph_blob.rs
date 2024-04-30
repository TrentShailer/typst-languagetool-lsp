use crate::range::Range;

use typst::syntax::{SyntaxKind, SyntaxNode};

pub fn get_paragraph_blob(paragraph: &[&SyntaxNode]) -> (String, Vec<Range>) {
    let mut blob = String::new();
    let mut node_ranges = Vec::with_capacity(paragraph.len());

    for node in paragraph {
        let mut node_text: String = String::new();

        match node.kind() {
            SyntaxKind::Space => {
                // linebreaks put in by the formatter
                // should be replaced with a space
                if node.text() == "\n" {
                    node_text = " ".to_string();
                } else {
                    node_text = node.text().to_string();
                }
            }
            SyntaxKind::Raw => {
                for child in node.children() {
                    node_text.push_str(child.text());
                }
            }
            _ => node_text = node.text().to_string(),
        }

        node_ranges.push(Range::new(blob.len(), node_text.len()));
        blob.push_str(&node_text);
    }

    (blob, node_ranges)
}
