use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use error_stack::{IntoReport, Result, ResultExt};
use ring::digest::{Context, Digest, SHA256};

use crate::{config, utils::CheckError, DISCLAIMER, PREFIX};

#[derive(Debug)]
pub(crate) struct HashError;

impl Display for HashError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unable to determine hash for provided file contents")
    }
}

impl std::error::Error for HashError {}

pub(crate) fn hash_reader<R: Read>(mut reader: R) -> Result<Digest, HashError> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader
            .read(&mut buffer)
            .into_report()
            .change_context(HashError)?;

        if count == 0 {
            break;
        }

        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub(crate) fn hash_file(path: &Path) -> Result<Digest, HashError> {
    let file = File::open(path).into_report().change_context(HashError)?;
    let reader = BufReader::new(file);

    hash_reader(reader)
}

const fn hex_from_digit(num: u8) -> char {
    if num < 10 {
        (b'0' + num) as char
    } else {
        // this makes sure that we use the correct character set
        // if we would just count up from 0 we would not get to the uppercase
        // letters due to how ascii works.
        (b'A' + num - 10) as char
    }
}

// This has been adopted from https://stackoverflow.com/a/48485777/9077988
pub(crate) fn to_hex(blob: &[u8]) -> String {
    let mut buffer = String::with_capacity(blob.len() * 2);

    for ch in blob {
        buffer.push(hex_from_digit(ch / 16));
        buffer.push(hex_from_digit(ch % 16));
    }

    buffer
}

pub(crate) fn hash_verify(source: &Path) -> Result<bool, CheckError> {
    let contents = std::fs::read_to_string(source)
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
            let expected = hash_file(&config::path().ok_or(CheckError::NotCargo).into_report()?)
                .change_context(CheckError::Hash)?;
            let expected = to_hex(expected.as_ref());

            Ok(actual == expected)
        }
    }
}
