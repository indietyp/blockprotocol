use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

use error_stack::{Report, Result, ResultExt};
use quote::quote;

use crate::{
    hash::hash_verify,
    utils,
    utils::{camel_case_to_pascal_case, CheckError},
    Config,
};

fn path() -> Result<PathBuf, CheckError> {
    let env = option_env!("CARGO_MANIFEST_DIR").ok_or_else(|| Report::new(CheckError::NotCargo))?;

    let path = Path::new(env)
        .parent()
        .ok_or_else(|| Report::new(CheckError::Path))?;

    Ok(path.join("syntax/src/kind.rs"))
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

pub(crate) fn generate(config: &Config) -> Result<(), GenerationError> {
    if check().change_context(GenerationError::Check)? {
        return Ok(());
    }

    let imports = quote! {
        #![allow(missing_docs, reason = "file is automatically generated")]

        use lexer::Kind;
        use num_derive::{FromPrimitive, ToPrimitive};
    };

    let kinds = config
        .kind
        .keys()
        .map(|key| quote::format_ident!("{}", camel_case_to_pascal_case(key)));

    let entries = kinds.clone().chain(
        config
            .syntax
            .keys()
            .map(|key| quote::format_ident!("{}", camel_case_to_pascal_case(key))),
    );

    let kinds = kinds.map(|kind| {
        let other = kind.clone();

        quote!(Kind::#kind => Self::#other)
    });

    let type_ = quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive, Copy, Clone, Hash)]
        pub enum SyntaxKind {
            #(#entries,)*

            Error,
            Tombstone
        }

        impl From<Kind> for SyntaxKind {
            fn from(value: Kind) -> Self {
                match value {
                    #(#kinds,)*
                    Kind::Error => Self::Error
                }
            }
        }
    };

    let stream = quote! {
        #imports

        #type_
    };

    utils::write(&path().change_context(GenerationError::Check)?, &stream)
        .change_context(GenerationError::Write)
}

pub(crate) fn check() -> Result<bool, CheckError> {
    let path = path()?;

    if !path.exists() {
        return Ok(false);
    }

    hash_verify(&path)
}
