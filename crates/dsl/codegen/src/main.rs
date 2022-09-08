use std::fmt::{Display, Formatter};

use error_stack::ResultExt;

mod config;
mod hash;
mod lexer;

#[derive(Debug)]
struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("error occurred during code generation")
    }
}

impl std::error::Error for Error {}

fn main() -> error_stack::Result<(), Error> {
    lexer::check().change_context(Error)?;

    Ok(())
}
