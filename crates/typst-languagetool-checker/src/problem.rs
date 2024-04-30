use languagetool_rust::check::Match;
use serde::Serialize;
use typst::syntax::{Source, SyntaxNode};
use wasm_bindgen::prelude::*;

use crate::{position::Position, range::Range};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Problem {
    pub range_start: Position,
    pub range_end: Position,
    pub message: String,
    pub replacements: Vec<String>,
    pub rule_category: String,
    pub rule_id: String,
}

impl Problem {
    pub fn try_from_match(
        source: &Source,
        paragraph: &[&SyntaxNode],
        node_ranges: &[Range],
        check_match: Match,
    ) -> Option<Self> {
        let match_local_range = Range::new(check_match.offset, check_match.length);

        // find the node that contains the start of the match
        let (node, node_local_range) =
            match find_match_node(&match_local_range, paragraph, node_ranges) {
                Some(v) => v,
                None => return None,
            };

        let node_range = match source.range(node.span()) {
            Some(v) => v,
            None => return None,
        };

        let match_start_offset = match_local_range.offset - node_local_range.offset;

        let global_start = node_range.start + match_start_offset;

        // TODO These are 0 indexed, vscode editor is 1 indexed
        let (range_start, range_end) =
            match get_match_range(source, global_start, match_local_range) {
                Some(v) => v,
                None => return None,
            };

        Some(Self {
            range_start,
            range_end,
            message: check_match.message,
            replacements: check_match
                .replacements
                .into_iter()
                .map(|v| v.value)
                .collect(),
            rule_category: check_match.rule.category.id,
            rule_id: check_match.rule.id,
        })
    }
}

fn get_match_range(
    source: &Source,
    global_start: usize,
    match_local_range: Range,
) -> Option<(Position, Position)> {
    let start_line = match source.byte_to_line(global_start) {
        Some(v) => v,
        None => return None,
    };
    let start_column = match source.byte_to_column(global_start) {
        Some(v) => v,
        None => return None,
    };
    let end_line = match source.byte_to_line(global_start + match_local_range.length) {
        Some(v) => v,
        None => return None,
    };
    let end_column = match source.byte_to_column(global_start + match_local_range.length) {
        Some(v) => v,
        None => return None,
    };
    Some((
        Position::new(start_line, start_column),
        Position::new(end_line, end_column),
    ))
}

fn find_match_node<'a>(
    match_range: &Range,
    nodes: &[&'a SyntaxNode],
    node_ranges: &'a [Range],
) -> Option<(&'a SyntaxNode, &'a Range)> {
    for (index, node) in nodes.iter().enumerate() {
        let node_range = match node_ranges.get(index) {
            Some(v) => v,
            None => return None,
        };

        if node_range.contains(match_range) {
            return Some((node, node_range));
        }
    }

    None
}
