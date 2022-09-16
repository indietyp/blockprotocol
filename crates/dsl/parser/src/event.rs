//! This has been adapted from
//! https://github.com/rust-lang/rust-analyzer/blob/6b163c301f70d0e1246fb898b5f5edcc4d03fa4c/crates/parser/src/event.rs

use std::{
    mem,
    ops::{Deref, DerefMut},
};

use error_stack::Report;

use crate::{error::ParserError, kind::SyntaxKind, output::Output};

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
    pub(crate) const fn tombstone() -> Self {
        Self::Start {
            kind: SyntaxKind::Tombstone,
            forward_parent: None,
        }
    }
}

pub(crate) struct Events {
    inner: Vec<Event>,
}

impl Events {
    pub(crate) fn new(events: Vec<Event>) -> Self {
        Self { inner: events }
    }

    pub(crate) fn process(self) -> Output {
        let Self { mut inner } = self;
        let mut res = Output::default();
        let mut forward_parents = Vec::new();

        for i in 0..inner.len() {
            match mem::replace(&mut inner[i], Event::tombstone()) {
                Event::Start {
                    kind,
                    forward_parent,
                } => {
                    // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
                    // in the normal control flow, the parent-child relation: `A -> B -> C`,
                    // while with the magic forward_parent, it writes: `C <- B <- A`.

                    // append `A` into parents.
                    forward_parents.push(kind);
                    let mut idx = i;
                    let mut fp = forward_parent;
                    while let Some(fwd) = fp {
                        idx += fwd as usize;
                        // append `A`'s forward_parent `B`
                        fp = match mem::replace(&mut inner[idx], Event::tombstone()) {
                            Event::Start {
                                kind,
                                forward_parent,
                            } => {
                                forward_parents.push(kind);
                                forward_parent
                            }
                            _ => unreachable!(),
                        };
                        // append `B`'s forward_parent `C` in the next stage.
                    }

                    #[expect(clippy::iter_with_drain, reason = "false-positive")]
                    for kind in forward_parents.drain(..).rev() {
                        if kind != SyntaxKind::Tombstone {
                            res.enter_node(kind);
                        }
                    }
                }
                Event::Finish => res.leave_node(),
                Event::Token { kind, n_raw_tokens } => {
                    res.token(kind, n_raw_tokens);
                }
                Event::Error(report) => res.error(report),
            }
        }

        res
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
