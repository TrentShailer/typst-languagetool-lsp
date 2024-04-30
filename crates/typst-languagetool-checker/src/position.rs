use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Default, Hash, PartialEq, PartialOrd, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
