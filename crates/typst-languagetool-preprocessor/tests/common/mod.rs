use typst::syntax::{SyntaxKind, SyntaxNode};

use std::fmt::Debug;

pub fn paragraphs_eq(results: &[Vec<&SyntaxNode>], expected: &[Vec<TestNode>]) -> bool {
    if results.len() != expected.len() {
        return false;
    }

    for paragraph_index in 0..results.len() {
        let a_paragraph = &results[paragraph_index];
        let b_paragraph = &expected[paragraph_index];

        if a_paragraph.len() != b_paragraph.len() {
            return false;
        }

        for node_index in 0..a_paragraph.len() {
            let a_node = a_paragraph[node_index];
            let b_node = &b_paragraph[node_index];

            if a_node.kind() != b_node.kind || a_node.text() != b_node.text {
                return false;
            }
        }
    }

    true
}

pub fn node(kind: SyntaxKind, text: &'static str) -> TestNode {
    TestNode { kind, text }
}

pub struct TestNode {
    pub kind: SyntaxKind,
    pub text: &'static str,
}

impl Debug for TestNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.kind, self.text)
    }
}
