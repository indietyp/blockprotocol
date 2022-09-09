use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use error_stack::{IntoReport, Report, Result, ResultExt};
use quote::{
    __private::{Ident, TokenStream},
    quote,
};

use crate::{
    config,
    config::{Config, Is},
    hash::{hash_file, to_hex},
    util::camel_case_to_pascal_case,
};

const DISCLAIMER: &str = "//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED";
const PREFIX: &str = "//! GENERATED WITH ";

#[derive(Debug)]
pub(crate) enum WriteError {
    Io,
    Hash,
    Check,
    Fmt,
}

impl Display for WriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io => {
                f.write_str("while trying to write to `lexer/src/kind.rs` an io error occurred")
            }
            Self::Hash => f.write_str("unable to hash contents"),
            Self::Check => f.write_str("searching for the `lexer/src/kind.rs` was not possible"),
            Self::Fmt => f.write_str("error while running `rustfmt` over the generated code"),
        }
    }
}

impl std::error::Error for WriteError {}

fn path() -> Result<PathBuf, CheckError> {
    let env = option_env!("CARGO_MANIFEST_DIR").ok_or_else(|| Report::new(CheckError::NotCargo))?;

    let path = Path::new(env)
        .parent()
        .ok_or_else(|| Report::new(CheckError::Path))?;

    Ok(path.join("lexer/src/kind.rs"))
}

fn write(stream: &TokenStream) -> Result<(), WriteError> {
    let path = path().change_context(WriteError::Check)?;

    let stream = stream.to_string();

    let mut file = File::create(&path)
        .into_report()
        .change_context(WriteError::Io)?;

    let hash = config::path()
        .ok_or(CheckError::NotCargo)
        .into_report()
        .change_context(WriteError::Check)?;

    let digest = hash_file(&hash).change_context(WriteError::Hash)?;
    let hash = to_hex(digest.as_ref());

    let contents = format!("{DISCLAIMER}\n{PREFIX}{hash}\n\n{stream}");

    file.write_all(contents.as_bytes())
        .into_report()
        .change_context(WriteError::Io)?;

    file.flush().into_report().change_context(WriteError::Io)?;
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
pub(crate) enum GenerationError {
    Check,
    Write,
}

impl Display for GenerationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Check => {
                f.write_str("while trying to check `lexer/src/kind.rs` an issue occurred")
            }
            Self::Write => f.write_str("writing of the file failed"),
        }
    }
}

impl std::error::Error for GenerationError {}

fn find_is<'a>(config: &'a Config, is: &'a Is) -> impl Iterator<Item = Ident> + 'a {
    config.kind.iter().filter_map(|(key, value)| {
        value
            .is
            .contains(is)
            .then(|| quote::format_ident!("{}", camel_case_to_pascal_case(key)))
    })
}

pub(crate) fn generate(config: &Config) -> Result<(), GenerationError> {
    let check = check().change_context(GenerationError::Check)?;
    if check {
        // the contents haven't changed so we don't need to regenerate
        return Ok(());
    }

    let imports = quote!(
        use logos::Logos;
        use num_derive::{FromPrimitive, ToPrimitive};
    );

    let entries = config.kind.iter().map(|(key, kind)| {
        let token = kind
            .token
            .as_ref()
            .map_or_else(|| quote!(), |token| quote!(#[token(#token)]));

        let regex = kind.regex.as_ref().map_or_else(
            || quote!(),
            |regex| {
                let regex = regex.iter().map(|regex| quote!(#[regex(#regex)]));

                quote!(#(#regex)*)
            },
        );

        let name = camel_case_to_pascal_case(key);
        let name = quote::format_ident!("{name}");

        quote!(
            #token
            #regex
            #name
        )
    });

    let trivia = find_is(config, &Is::Trivia);
    let literals = find_is(config, &Is::Literal);
    let infix_ops = find_is(config, &Is::InfixOp);
    let prefix_ops = find_is(config, &Is::PrefixOp);
    let postfix_ops = find_is(config, &Is::PostfixOp);

    let kinds = quote!(
        #[derive(Logos, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive, Copy, Clone)]
        pub enum Kind {
            #(#entries,)*

            #[error]
            Error
        }

        impl Kind {
            pub const fn trivia(&self) -> &'static [Self] {
                &[#(Self::#trivia),*]
            }

            pub fn is_trivia(&self) -> bool {
                self.trivia().contains(self)
            }

            pub const fn literals(&self) -> &'static [Self] {
                &[#(Self::#literals),*]
            }

            pub fn is_literal(&self) -> bool {
                self.literals().contains(self)
            }

            pub const fn infix_ops(&self) -> &'static [Self] {
                &[#(Self::#infix_ops),*]
            }

            pub fn is_infix_op(&self) -> bool {
                self.infix_ops().contains(self)
            }

            pub const fn prefix_ops(&self) -> &'static [Self] {
                &[#(Self::#prefix_ops),*]
            }

            pub fn is_prefix_op(&self) -> bool {
                self.prefix_ops().contains(self)
            }

            pub const fn postfix_ops(&self) -> &'static [Self] {
                &[#(Self::#postfix_ops),*]
            }

            pub fn is_postfix_op(&self) -> bool {
                self.postfix_ops().contains(self)
            }
        }
    );

    let stream = quote! {
        #imports

        #kinds
    };

    write(&stream).change_context(GenerationError::Write)
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
            Self::Io => f.write_str("could not access `kind.rs` file"),
            Self::Hash => f.write_str("could not hash file contents of `types.toml`"),
        }
    }
}

impl std::error::Error for CheckError {}

pub(crate) fn check() -> Result<bool, CheckError> {
    let path = path()?;

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
                let expected =
                    hash_file(&config::path().ok_or(CheckError::NotCargo).into_report()?)
                        .change_context(CheckError::Hash)?;
                let expected = to_hex(expected.as_ref());

                Ok(actual == expected)
            }
        }
    } else {
        Ok(false)
    }
}
