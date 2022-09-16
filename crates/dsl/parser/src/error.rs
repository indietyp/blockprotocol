use std::{
    any::Demand,
    fmt::{Debug, Display, Formatter},
};

use either::Either;
use error_stack::Report;
use text_size::TextRange;

use crate::SyntaxKind;

#[derive(Debug)]
pub struct ParserError;

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unable to parse input")
    }
}

impl std::error::Error for ParserError {}

pub struct Note(String);

impl Note {
    pub(crate) fn new(note: impl Into<String>) -> Self {
        Self(note.into())
    }
}

pub struct Label {
    message: String,
    // u32 is the relative position of the token, this ensures that we're able to retrace
    // which token caused the issue.
    span: Either<usize, TextRange>,
}

impl Label {
    pub(crate) fn new(message: impl Into<String>, span: Either<usize, TextRange>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

#[derive(Debug)]
pub enum Expected {
    Kind(SyntaxKind),
    Name,
    Ident,
    String,
    Expr,
}

impl Display for Expected {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kind(kind) => Debug::fmt(kind, f),
            Self::Name => f.write_str("name"),
            Self::Ident => f.write_str("identifier"),
            Self::String => f.write_str("string"),
            Self::Expr => f.write_str("expression"),
        }
    }
}

#[derive(Debug)]
pub struct ExpectedError {
    pos: usize,
    kind: Expected,
}

impl Display for ExpectedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("expected ")?;
        Display::fmt(&self.kind, f)
    }
}

impl std::error::Error for ExpectedError {
    fn provide<'a>(&'a self, demand: &mut Demand<'a>) {
        demand.provide_value(Label::new(format!("{self}"), Either::Left(self.pos)));
    }
}

impl ExpectedError {
    pub(crate) const fn new(pos: usize, kind: Expected) -> Self {
        Self { pos, kind }
    }

    pub(crate) fn report(pos: usize, kind: Expected) -> Report<Self> {
        Report::new(Self::new(pos, kind))
    }
}

#[derive(Debug)]
pub struct UnmatchedError {
    pos: usize,
    closing: SyntaxKind,
}

impl Display for UnmatchedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unmatched ")?;
        Debug::fmt(&self.closing, f)
    }
}

impl std::error::Error for UnmatchedError {
    fn provide<'a>(&'a self, demand: &mut Demand<'a>) {
        demand.provide_value(Label::new(format!("{self}"), Either::Left(self.pos)));
    }
}

impl UnmatchedError {
    pub(crate) const fn new(pos: usize, closing: SyntaxKind) -> Self {
        Self { pos, closing }
    }

    pub(crate) fn report(pos: usize, closing: SyntaxKind) -> Report<Self> {
        Report::new(Self::new(pos, closing))
    }
}
