use std::{
    fmt::{Display, Formatter},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use error_stack::{IntoReport, Report, Result, ResultExt};
use quote::{__private::Ident, quote};

use crate::{
    config,
    config::{Config, Is},
    hash::{hash_file, hash_verify, to_hex},
    utils,
    utils::{camel_case_to_pascal_case, CheckError},
    DISCLAIMER, PREFIX,
};

fn path() -> Result<PathBuf, CheckError> {
    let env = option_env!("CARGO_MANIFEST_DIR").ok_or_else(|| Report::new(CheckError::NotCargo))?;

    let path = Path::new(env)
        .parent()
        .ok_or_else(|| Report::new(CheckError::Path))?;

    Ok(path.join("lexer/src/kind.rs"))
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
        #![allow(missing_docs, reason = "file is automatically generated")]

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

    let type_ = quote!(
        #[derive(Logos, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive, Copy, Clone)]
        pub enum Kind {
            #(#entries,)*

            #[error]
            Error
        }

        impl Kind {
            #[must_use]
            pub const fn trivia(&self) -> &'static [Self] {
                &[#(Self::#trivia),*]
            }

            #[must_use]
            pub fn is_trivia(&self) -> bool {
                self.trivia().contains(self)
            }

            #[must_use]
            pub const fn literals(&self) -> &'static [Self] {
                &[#(Self::#literals),*]
            }

            #[must_use]
            pub fn is_literal(&self) -> bool {
                self.literals().contains(self)
            }

            #[must_use]
            pub const fn infix_ops(&self) -> &'static [Self] {
                &[#(Self::#infix_ops),*]
            }

            #[must_use]
            pub fn is_infix_op(&self) -> bool {
                self.infix_ops().contains(self)
            }

            #[must_use]
            pub const fn prefix_ops(&self) -> &'static [Self] {
                &[#(Self::#prefix_ops),*]
            }

            #[must_use]
            pub fn is_prefix_op(&self) -> bool {
                self.prefix_ops().contains(self)
            }

            #[must_use]
            pub const fn postfix_ops(&self) -> &'static [Self] {
                &[#(Self::#postfix_ops),*]
            }

            #[must_use]
            pub fn is_postfix_op(&self) -> bool {
                self.postfix_ops().contains(self)
            }
        }
    );

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
