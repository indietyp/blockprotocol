use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use error_stack::{IntoReport, Result, ResultExt};
use ring::digest::{Context, Digest, SHA256};

#[derive(Debug)]
pub(crate) struct HashError;

impl Display for HashError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unable to determine hash for provided file contents")
    }
}

impl std::error::Error for HashError {}

pub(crate) fn hash_file(path: &Path) -> Result<Digest, HashError> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    let file = File::open(path).into_report().change_context(HashError)?;
    let mut reader = BufReader::new(file);
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

fn hex_from_digit(num: u8) -> char {
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
