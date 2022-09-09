//! This crate isn't directly consumed, but instead used internally, to generate all related files
//! from a single source of truth: the `types.toml` file.
//!
//! Why use a codegen approach and not a proc-macro?
//! The types.toml is only going to be changed a few times through it's history and the overhead of
//! a proc-macro for such an isolated case is too high.
//!
//! The `types.toml` file enables us to generate all code needed once and check for updates via the
//! prefix of a file.

#![warn(
    missing_docs,
    unreachable_pub,
    clippy::pedantic,
    clippy::nursery,
    clippy::undocumented_unsafe_blocks
)]
#![allow(clippy::redundant_pub_crate)] // This would otherwise clash with `unreachable_pub`
#![allow(clippy::module_name_repetitions)]

use std::fmt::{Display, Formatter};

use error_stack::ResultExt;

use crate::config::Config;

pub(crate) mod config;
mod hash;
mod lexer;
mod util;

#[derive(Debug)]
struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("error occurred during code generation")
    }
}

impl std::error::Error for Error {}

fn main() -> error_stack::Result<(), Error> {
    // TODO: cleanup and check (clap) + Makefile.toml
    let config = Config::load().change_context(Error)?;

    lexer::generate(&config).change_context(Error)?;

    Ok(())
}
