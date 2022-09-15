//! This module takes the lexed output from the `lexer` crate and transforms it into a CST and AST
//! (which is built upon the CST).

#![feature(lint_reasons)]
#![warn(
    missing_docs,
    unreachable_pub,
    clippy::pedantic,
    clippy::nursery,
    clippy::undocumented_unsafe_blocks
)]
#![allow(clippy::redundant_pub_crate)] // This would otherwise clash with `unreachable_pub`
#![allow(clippy::module_name_repetitions)]

mod error;
mod kind;
mod tree;
