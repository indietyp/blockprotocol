use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Write,
    path::Path,
    process::Command,
};

use error_stack::{IntoReport, ResultExt};
use quote::__private::TokenStream;

use crate::{
    config,
    hash::{hash_file, to_hex},
    DISCLAIMER, PREFIX,
};

pub(crate) fn camel_case_to_pascal_case(value: &str) -> String {
    value
        .split('-')
        .map(ToOwned::to_owned)
        .map(|mut value| {
            if let Some(first) = value.get_mut(0..1) {
                first.make_ascii_uppercase();
            }

            value
        })
        .collect::<Vec<_>>()
        .concat()
}

#[derive(Debug)]
pub(crate) enum WriteError {
    Io,
    Hash,
    Fmt,
    NotCargo,
}

impl Display for WriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotCargo => f.write_str("`codegen` needs to be executed using `rust run`"),
            Self::Io => {
                f.write_str("while trying to write to `lexer/src/kind.rs` an io error occurred")
            }
            Self::Hash => f.write_str("unable to hash contents"),
            Self::Fmt => f.write_str("error while running `rustfmt` over the generated code"),
        }
    }
}

impl std::error::Error for WriteError {}

pub(crate) fn write(path: &Path, stream: &TokenStream) -> error_stack::Result<(), WriteError> {
    let stream = stream.to_string();

    let mut file = File::create(path)
        .into_report()
        .change_context(WriteError::Io)?;

    let hash = config::path().ok_or(WriteError::NotCargo).into_report()?;

    let digest = hash_file(&hash).change_context(WriteError::Hash)?;
    let hash = to_hex(digest.as_ref());

    let contents = format!("{DISCLAIMER}\n{PREFIX}{hash}\n\n{stream}");

    file.write_all(contents.as_bytes())
        .into_report()
        .change_context(WriteError::Io)?;

    file.flush().into_report().change_context(WriteError::Io)?;
    file.sync_all()
        .into_report()
        .change_context(WriteError::Io)?;

    // make sure we dropped the file before we format
    drop(file);

    let out = Command::new("sh")
        .arg("-c")
        .arg(format!("cargo fmt -- {}", path.to_str().unwrap()))
        .output()
        .into_report()
        .change_context(WriteError::Fmt)?;

    println!("{out:?}");

    Ok(())
}

#[derive(Debug)]
pub(crate) enum CheckError {
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
            Self::Io => f.write_str("could not access file"),
            Self::Hash => f.write_str("could not hash file contents of `types.toml`"),
        }
    }
}

impl std::error::Error for CheckError {}
