//! This module takes the CST from the `parser` crate and transforms it into an AST

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
mod parse;
mod tree;
