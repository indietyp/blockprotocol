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

mod error;
mod event;
mod input;
#[macro_use]
pub mod kind;
mod lexed_str;
mod marker;
mod output;
mod parser;
mod token_set;
