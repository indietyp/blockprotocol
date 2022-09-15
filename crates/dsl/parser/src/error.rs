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
    pub fn new(note: impl Into<String>) -> Self {
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
    pub fn new(message: impl Into<String>, span: Either<usize, TextRange>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

#[derive(Debug)]
pub struct ExpectedError {
    pos: usize,
    kind: SyntaxKind,
}

impl Display for ExpectedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("expected ")?;
        Debug::fmt(&self.kind, f)
    }
}

impl std::error::Error for ExpectedError {
    fn provide<'a>(&'a self, demand: &mut Demand<'a>) {
        demand.provide_value(Label::new(format!("{self}"), Either::Left(self.pos)));
    }
}

impl ExpectedError {
    pub(crate) fn new(pos: usize, kind: SyntaxKind) -> Self {
        Self { pos, kind }
    }

    pub(crate) fn report(pos: usize, kind: SyntaxKind) -> Report<Self> {
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
    pub(crate) fn new(pos: usize, closing: SyntaxKind) -> Self {
        Self { pos, closing }
    }

    pub(crate) fn report(pos: usize, closing: SyntaxKind) -> Report<Self> {
        Report::new(Self::new(pos, closing))
    }
}

#[derive(Debug)]
pub struct ExpectedIdentifierError {
    pos: usize,
}

impl Display for ExpectedIdentifierError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("expected identifier")
    }
}

impl std::error::Error for ExpectedIdentifierError {
    fn provide<'a>(&'a self, demand: &mut Demand<'a>) {
        demand.provide_value(Label::new(
            format!("expected identifer"),
            Either::Left(self.pos),
        ));
    }
}

impl ExpectedIdentifierError {
    pub(crate) fn new(pos: usize) -> Self {
        Self { pos }
    }

    pub(crate) fn report(pos: usize) -> Report<Self> {
        Report::new(Self::new(pos))
    }
}
