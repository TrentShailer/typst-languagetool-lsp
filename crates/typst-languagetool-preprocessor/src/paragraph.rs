use typst::syntax::{SyntaxKind, SyntaxNode};

/// Extract the paragraphs of text from a syntax tree
pub fn get_paragraphs(root: &SyntaxNode) -> Vec<Vec<&SyntaxNode>> {
    let (mut paragraphs, partial_paragraph) = recursively_build_paragraphs(root, Vec::new());

    if !partial_paragraph.is_empty() {
        paragraphs.push(partial_paragraph)
    }

    paragraphs
}

/// The type of a given syntax node
enum NodeType {
    /// A node that marks the end of a paragraph
    ParagraphTerminator,
    /// A node that marks the end of a paragraph and all child paragraphs
    ParagraphContainer,
    /// A node that does not contain any paragraphs
    NonParagraph,
    /// A node that is a part of a paragraph
    Capture,
    /// Any other node
    Normal,
}

impl From<SyntaxKind> for NodeType {
    fn from(value: SyntaxKind) -> Self {
        match value {
            SyntaxKind::Parbreak | SyntaxKind::Eof => NodeType::ParagraphTerminator,

            SyntaxKind::Heading | SyntaxKind::Linebreak | SyntaxKind::Named => {
                NodeType::ParagraphContainer
            }

            SyntaxKind::Math | SyntaxKind::SetRule | SyntaxKind::CodeBlock => {
                NodeType::NonParagraph
            }

            SyntaxKind::Text | SyntaxKind::SmartQuote | SyntaxKind::Space | SyntaxKind::Raw => {
                NodeType::Capture
            }

            _ => NodeType::Normal,
        }
    }
}

fn recursively_build_paragraphs<'a>(
    node: &'a SyntaxNode,
    current_paragraph: Vec<&'a SyntaxNode>,
) -> (Vec<Vec<&'a SyntaxNode>>, Vec<&'a SyntaxNode>) {
    let mut paragraphs = vec![];
    let mut current_paragraph = current_paragraph;

    let node_type: NodeType = node.kind().into();

    match node_type {
        // Terminate current paragraph
        NodeType::ParagraphTerminator | NodeType::ParagraphContainer => {
            if !current_paragraph.is_empty() {
                paragraphs.push(current_paragraph);
                current_paragraph = Vec::new();
            }
        }

        // Terminate current paragraph, immediate return
        NodeType::NonParagraph => {
            if !current_paragraph.is_empty() {
                paragraphs.push(current_paragraph);
            }

            return (paragraphs, vec![]);
        }

        // Add self to current paragraph, immediately return
        NodeType::Capture => {
            // Don't push witespace to an empty paragraph
            if let SyntaxKind::Space = node.kind() {
                if current_paragraph.is_empty() {
                    return (vec![], current_paragraph);
                }
            }

            current_paragraph.push(node);
            return (vec![], current_paragraph);
        }
        _ => {}
    }

    // Recurse through children, building paragraphs
    let mut prev_node_kind = SyntaxKind::None;
    let mut iter = node.children().peekable();
    while let Some(child) = iter.next() {
        // Ignore a raw surrounded by parbreaks
        if prev_node_kind == SyntaxKind::Parbreak && child.kind() == SyntaxKind::Raw {
            let maybe_next = iter.peek();
            if let Some(next) = maybe_next {
                if next.kind() == SyntaxKind::Parbreak {
                    prev_node_kind = child.kind();
                    continue;
                }
            }
        }

        let (mut child_groups, new_current_group) =
            recursively_build_paragraphs(child, current_paragraph);

        if !child_groups.is_empty() {
            paragraphs.append(&mut child_groups);
        }

        current_paragraph = new_current_group;
        prev_node_kind = child.kind();
    }

    // If node is a container, child paragraphs should terminate here
    if let NodeType::ParagraphContainer = node_type {
        if !current_paragraph.is_empty() {
            paragraphs.push(current_paragraph);
            current_paragraph = Vec::new();
        }
    }

    (paragraphs, current_paragraph)
}
