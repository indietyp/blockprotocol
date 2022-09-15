//! This has been adapted from
//! https://github.com/rust-lang/rust-analyzer/blob/6b163c301f70d0e1246fb898b5f5edcc4d03fa4c/crates/parser/src/event.rs

use std::ops::{Deref, DerefMut};

use error_stack::Report;

use crate::{error::ParserError, kind::SyntaxKind};

pub(crate) enum Event {
    /// This event signifies the start of the node.
    /// It should be either abandoned (in which case the
    /// `kind` is `Tombstone`, and the event is ignored),
    /// or completed via a `Finish` event.
    ///
    /// All tokens between a `Start` and a `Finish` would
    /// become the children of the respective node.
    ///
    /// For left-recursive syntactic constructs, the parser produces
    /// a child node before it sees a parent. `forward_parent`
    /// saves the position of current event's parent.
    ///
    /// Consider this path
    ///
    /// foo::bar
    ///
    /// The events for it would look like this:
    ///
    /// ```text
    /// START(PATH) IDENT('foo') FINISH START(PATH) T![::] IDENT('bar') FINISH
    ///       |                          /\
    ///       |                          |
    ///       +------forward-parent------+
    /// ```
    ///
    /// And the tree would look like this
    ///
    /// ```text
    ///    +--PATH---------+
    ///    |   |           |
    ///    |   |           |
    ///    |  '::'       'bar'
    ///    |
    ///   PATH
    ///    |
    ///   'foo'
    /// ```
    ///
    /// See also `CompletedMarker::precede`.
    Start {
        kind: SyntaxKind,
        forward_parent: Option<u32>,
    },

    /// Complete the previous `Start` event
    Finish,

    /// Produce a single leaf-element.
    /// `n_raw_tokens` is used to glue complex contextual tokens.
    /// For example, lexer tokenizes `>>` as `>`, `>`, and
    /// `n_raw_tokens = 2` is used to produced a single `>>`.
    Token {
        kind: SyntaxKind,
        n_raw_tokens: u8,
    },

    Error(Report<ParserError>),
}

impl Event {
    pub(crate) fn tombstone() -> Self {
        Event::Start {
            kind: SyntaxKind::Tombstone,
            forward_parent: None,
        }
    }
}

pub(crate) struct Events {
    inner: Vec<Event>,
}

impl Events {
    pub(crate) fn process(self) -> Output {
        
    }
}

impl Deref for Events {
    type Target = Vec<Event>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Events {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
