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
