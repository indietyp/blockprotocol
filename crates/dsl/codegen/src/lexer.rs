use std::{
    fmt::{Display, Formatter, Write},
    path::Path,
};

use error_stack::{IntoReport, IteratorExt, Report, Result, ResultExt};
use quote::quote;

use crate::hash::{hash_file, to_hex};

const DISCLAIMER: &str = "//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED";
const PREFIX: &str = "//! GENERATED WITH ";

#[derive(Debug)]
pub(crate) enum GenerationError {
    Check,
}

impl Display for GenerationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationError::Check => {
                f.write_str("while trying to check `lexer.rs` an issue occurred")
            }
        }
    }
}

impl std::error::Error for GenerationError {}

fn generate() -> Result<(), GenerationError> {
    let check = check().change_context(GenerationError::Check)?;
    if check {
        // the contents haven't changed so we don't need to regenerate
        return Ok(());
    }

    let imports = quote!(
        use logos::Logos;
        use num_derive::{FromPrimitive, ToPrimitive}
    );

    let kinds = quote!(
        #[derive(Logos, Debug, PartialEq, ToPrimitive, Copy, Clone)]
        pub enum Kind {
            // TODO
        }
    );

    todo!()
}

#[derive(Debug)]
pub enum CheckError {
    NotCargo,
    Path,
    Io,
    Hash,
}

impl Display for CheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotCargo => f.write_str("codegen can only be executed using `cargo run`"),
            Self::Path => {
                f.write_str("`codegen` was unable to determine the path to the `lexer` package")
            }
            Self::Io => f.write_str("could not access `kind.rs` file"),
            Self::Hash => f.write_str("could not hash file contents of `types.toml`"),
        }
    }
}

impl std::error::Error for CheckError {}

pub(crate) fn check() -> Result<bool, CheckError> {
    let env = option_env!("CARGO_MANIFEST_DIR").ok_or_else(|| Report::new(CheckError::NotCargo))?;

    let path = Path::new(env)
        .parent()
        .ok_or_else(|| Report::new(CheckError::Path))?;

    let path = path.join("lexer/src/kind.rs");

    if path.exists() {
        let contents = std::fs::read_to_string(path.as_path())
            .into_report()
            .change_context(CheckError::Io)?;
        let mut lines = contents.lines();

        let disclaimer = lines.next();
        let hash = lines.next();

        if disclaimer.is_none() || disclaimer != Some(DISCLAIMER) {
            return Ok(false);
        }

        match hash {
            None => Ok(false),
            Some(hash) if !hash.starts_with(PREFIX) => Ok(false),
            Some(hash) => {
                // SAFETY: previous arm guarantees that there's a prefix
                let actual = hash.strip_prefix(PREFIX).unwrap();
                let expected = hash_file(path.as_path()).change_context(CheckError::Hash)?;
                let expected = to_hex(expected.as_ref());

                Ok(actual == expected)
            }
        }
    } else {
        Ok(false)
    }
}
