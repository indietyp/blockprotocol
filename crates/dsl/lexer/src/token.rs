use text_size::TextRange;

use crate::kind::Kind;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Token<'a> {
    kind: Kind,
    text: &'a str,
    range: TextRange,
}

impl<'a> Token<'a> {
    pub(crate) fn new(kind: Kind, text: &'a str, range: TextRange) -> Self {
        Self { kind, text, range }
    }
}

impl Token<'_> {
    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn range(&self) -> TextRange {
        self.range
    }
}
