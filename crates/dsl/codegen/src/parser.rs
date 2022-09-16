use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

use error_stack::{Report, Result, ResultExt};
use quote::{__private::TokenTree, format_ident, quote};
use syn::__private::TokenStream;

use crate::{
    config::Is,
    hash::hash_verify,
    lexer::find_is,
    utils,
    utils::{camel_case_to_pascal_case, CheckError},
    Config,
};

fn path() -> Result<PathBuf, CheckError> {
    let env = option_env!("CARGO_MANIFEST_DIR").ok_or_else(|| Report::new(CheckError::NotCargo))?;

    let path = Path::new(env)
        .parent()
        .ok_or_else(|| Report::new(CheckError::Path))?;

    Ok(path.join("parser/src/kind.rs"))
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
                f.write_str("while trying to check `parser/src/kind.rs` an issue occurred")
            }
            Self::Write => f.write_str("writing of the file failed"),
        }
    }
}

impl std::error::Error for GenerationError {}

fn shortcuts(config: &Config) -> impl Iterator<Item = quote::__private::TokenStream> + '_ {
    config
        .kind
        .iter()
        .map(|(key, value)| (key, value.shortcut.as_ref().or(value.token.as_ref())))
        .chain(
            config
                .syntax
                .iter()
                .map(|(key, value)| (key, value.shortcut.as_ref())),
        )
        .filter_map(|(key, token)| {
            token.map(|token| {
                let key = format_ident!("{}", camel_case_to_pascal_case(key));

                let tree = syn::parse_str::<syn::__private::TokenStream2>(token).unwrap();

                quote!([#tree] => {$crate :: SyntaxKind :: #key})
            })
        })
}

fn composite(config: &Config) -> quote::__private::TokenStream {
    let composite = config
        .syntax
        .iter()
        .filter_map(|(key, value)| value.composite.as_ref().map(|composite| (key, composite)));

    let nth_at = composite.clone().map(|(key, composite)| {
        let ident = format_ident!("{}", camel_case_to_pascal_case(key));
        match &composite[..] {
            [a, b] => {
                let a = syn::parse_str::<syn::__private::TokenStream2>(a).unwrap();
                let b = syn::parse_str::<syn::__private::TokenStream2>(b).unwrap();

                quote!(SyntaxKind::#ident => self.at_composite2(n, T![#a], T![#b]))
            }
            [a, b, c] => {
                let a = syn::parse_str::<syn::__private::TokenStream2>(a).unwrap();
                let b = syn::parse_str::<syn::__private::TokenStream2>(b).unwrap();
                let c = syn::parse_str::<syn::__private::TokenStream2>(c).unwrap();

                quote!(SyntaxKind::#ident => self.at_composite3(n, T![#a], T![#b], T![#c]))
            }
            _ => panic!("can only composite from 2 or 3"),
        }
    });

    let n_raw_tokens = composite.map(|(key, composite)| {
        let ident = format_ident!("{}", camel_case_to_pascal_case(key));
        let len = composite.len() as u8;

        quote!(SyntaxKind::#ident => #len)
    });

    quote! {
        impl SyntaxKind {
            pub(crate) fn n_raw_tokens(&self) -> u8 {
                match self {
                    #(#n_raw_tokens,)*
                    _ => 1
                }
            }
        }

        impl Parser<'_> {
            pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
                match kind {
                    #(#nth_at,)*
                    _ => self.inp.kind(self.pos + n) == kind
                }
            }
        }
    }
}

fn precedence() -> quote::__private::TokenStream {
    quote! {
        impl SyntaxKind {
            fn precedence_infix() -> Option<(Associativity, Precedence)> {

            }

            fn precedence_prefix() -> Option<Precedence> {

            }

            fn precedence_postfix() -> Option<Precedence> {

            }
        }
    }
}

pub(crate) fn generate(config: &Config) -> Result<(), GenerationError> {
    if check().change_context(GenerationError::Check)? {
        return Ok(());
    }

    let imports = quote! {
        #![allow(missing_docs, reason = "file is automatically generated")]

        use lexer::Kind;
        use num_derive::{FromPrimitive, ToPrimitive};

        use crate::parser::Parser;
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

    let trivia = find_is(config, &Is::Trivia);

    let shortcuts = shortcuts(config);

    let contextual = config.syntax.iter().filter_map(|(key, value)| {
        value.contextual.as_ref().map(|ident| {
            let key = format_ident!("{}", camel_case_to_pascal_case(key));
            quote!(#ident => Self::#key)
        })
    });

    let composite = composite(config);

    let type_ = quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive, Copy, Clone, Hash)]
        pub enum SyntaxKind {
            #(#entries,)*

            EndOfFile,
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

        impl SyntaxKind {
            #[must_use]
            pub const fn trivia(&self) -> &'static [Self] {
                &[#(Self::#trivia),*]
            }

            #[must_use]
            pub fn is_trivia(&self) -> bool {
                self.trivia().contains(self)
            }

            pub fn from_contextual_keyword(ident: &str) -> Option<SyntaxKind> {
                let kw = match ident {
                    #(#contextual,)*
                    _ => return None,
                };
                Some(kw)
            }
        }

        macro_rules! T {
            #(#shortcuts;)*
        }

        #composite
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
