#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct Range {
    pub offset: usize,
    pub length: usize,
}

impl Range {
    pub fn new(offset: usize, length: usize) -> Self {
        Self { offset, length }
    }

    pub fn contains(&self, other: &Range) -> bool {
        if self.offset >= other.offset + other.length {
            return false;
        }

        if self.offset + self.length < other.offset {
            return false;
        }

        return true;
    }
}
