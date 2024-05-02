use crate::range::Range;

use typst::syntax::{SyntaxKind, SyntaxNode};

pub fn get_paragraph_blob(paragraph: &[&SyntaxNode]) -> (String, Vec<Range>) {
    dbg!(&paragraph);

    let mut blob = String::new();
    let mut node_ranges = Vec::with_capacity(paragraph.len());

    let mut iterator = paragraph.iter().peekable();

    while let Some(node) = iterator.next() {
        let mut node_text: String = String::new();

        // Due to formatter, a whitespace node should be interpreted as a single space
        // and all following whitespace should be ignored
        if node.kind() == SyntaxKind::Space {
            node_ranges.push(Range::new(blob.len(), 1));
            blob.push_str(" ");

            while let Some(next) = iterator.peek() {
                if next.kind() == SyntaxKind::Space {
                    node_ranges.push(Range::new(blob.len(), 0));
                    iterator.next();
                } else {
                    break;
                }
            }
            continue;
        }

        node_text.push_str(node.text());
        for child in node.children() {
            node_text.push_str(child.text());
        }

        node_ranges.push(Range::new(blob.len(), node_text.len()));
        blob.push_str(&node_text);
    }

    dbg!(&blob);

    /* for node in paragraph {
           let mut node_text: String = String::new();

           match node.kind() {
               SyntaxKind::Space => {
                   // linebreaks put in by the formatter
                   // should be replaced with a space
                   if node.text() == "\n" {
                       node_text = " ".to_string();
                   } else {
                       // if whitespace text contains any spaces, collapse to single space.
                       if node.text().to_string().contains(' ') {
                           // node_text = " ".to_string();
                           node_text = node.text().to_string();
                       } else {
                           node_text = node.text().to_string();
                       }
                   }
               }
               _ => {
                   node_text.push_str(node.text());
                   for child in node.children() {
                       node_text.push_str(child.text());
                   }
               }
           }

           node_ranges.push(Range::new(blob.len(), node_text.len()));
           blob.push_str(&node_text);
       }
    */
    (blob, node_ranges)
}
