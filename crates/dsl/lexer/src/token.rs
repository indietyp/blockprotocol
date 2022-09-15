use text_size::TextRange;

use crate::kind::Kind;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Token<'a, T: Copy = Kind> {
    kind: T,
    text: &'a str,
    range: TextRange,
}

impl<'a, T: Copy> Token<'a, T> {
    pub const fn new(kind: T, text: &'a str, range: TextRange) -> Self {
        Self { kind, text, range }
    }
}

impl<'a, T: Copy> Token<'a, T> {
    pub const fn kind(&self) -> T {
        self.kind
    }

    pub const fn text(&self) -> &'a str {
        self.text
    }

    pub const fn range(&self) -> TextRange {
        self.range
    }
}
