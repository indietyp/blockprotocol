//! The Pika parser
//!
//! The parser doesn't know about concrete representation of tokens and syntax
//! trees. Abstract [`TokenSource`] and [`TreeSink`] traits are used instead. As
//! a consequence, this crate does not contain a lexer.
//!
//! The [`Parser`] struct from the [`parser`] module is a cursor into the
//! sequence of tokens.
//! Parsing routines use [`Parser`] to inspect current state and advance the parsing.
//!
//! The actual parsing happens in the [`grammar`] module.
//!
//! Tests for this crate live in the `syntax` crate.
//!
//! TODO: semicolons on item level should be optional
//!
//! [`Parser`]: crate::parser::Parser
#![feature(lint_reasons)]
#![feature(error_generic_member_access)]
#![feature(provide_any)]
#![warn(
    missing_docs,
    unreachable_pub,
    clippy::pedantic,
    clippy::nursery,
    clippy::undocumented_unsafe_blocks
)]
#![allow(clippy::redundant_pub_crate)] // This would otherwise clash with `unreachable_pub`
#![allow(clippy::module_name_repetitions)]

pub use kind::SyntaxKind;

use crate::{input::Input, output::Output};

mod error;
mod event;
mod input;
#[macro_use]
pub mod kind;
mod grammar;
mod lexed_str;
mod marker;
mod output;
mod parser;
mod token_set;

/// Parse a prefix of the input as a given syntactic construct.
///
/// This is used by macro-by-example parser to implement things like `$i:item`
/// and the naming of variants follows the naming of macro fragments.
///
/// Note that this is generally non-optional -- the result is intentionally not
/// `Option<Output>`. The way MBE work, by the time we *try* to parse `$e:expr`
/// we already commit to expression. In other words, this API by design can't be
/// used to implement "rollback and try another alternative" logic.
#[derive(Debug)]
pub enum PrefixEntryPoint {
    // /// A block is current unused, but will be available in user defined functions
    // /// (if ever supported)
    // Block,
    // /// Statements are currently unused, but will be available in user defined functions
    // /// (if ever supported)
    // Stmt,
    /// Type declarations like:
    /// ```text
    /// {
    ///     record,
    ///     type
    /// }
    /// ```
    Ty,
    /// Expressions like:
    /// ```text
    /// "example"
    /// ```
    Expr,
    /// Paths like:
    /// ```text
    /// impl::config::abc
    /// ```
    Path,
    /// Items like:
    /// ```text
    /// prop "X": {}
    /// ```
    Item,
    /// This includes things like:
    /// ```text
    /// #[version = 12]
    /// ```
    Attribute,
}

impl PrefixEntryPoint {
    pub fn parse(&self, input: &Input) -> Output {
        let entry_point: fn(&'_ mut parser::Parser<'_>) = match self {
            PrefixEntryPoint::Ty => grammar::entry::prefix::ty,
            PrefixEntryPoint::Expr => grammar::entry::prefix::expr,
            PrefixEntryPoint::Path => grammar::entry::prefix::path,
            PrefixEntryPoint::Item => grammar::entry::prefix::item,
            PrefixEntryPoint::Attribute => grammar::entry::prefix::attribute,
        };
        let mut p = parser::Parser::new(input);
        entry_point(&mut p);
        let events = p.finish();

        events.process()
    }
}

/// A parsing function for a specific braced-block.
pub struct Reparser(fn(&mut parser::Parser<'_>));

impl Reparser {
    /// If the node is a braced block, return the corresponding `Reparser`.
    pub fn for_node(
        node: SyntaxKind,
        first_child: Option<SyntaxKind>,
        parent: Option<SyntaxKind>,
    ) -> Option<Reparser> {
        grammar::reparser(node, first_child, parent).map(Reparser)
    }

    /// Re-parse given tokens using this `Reparser`.
    ///
    /// Tokens must start with `{`, end with `}` and form a valid brace
    /// sequence.
    pub fn parse(self, tokens: &Input) -> Output {
        let Reparser(r) = self;
        let mut p = parser::Parser::new(tokens);
        r(&mut p);

        let events = p.finish();

        events.process()
    }
}
