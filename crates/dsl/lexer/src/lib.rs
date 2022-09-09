//! Lexer of the DSL, it's relies heavily on the logos crate to lex all input and returns
//! a stream of tokens which have been lexed.
//!
//! Most of the source code (located in `kind.rs`) has been generated through the codegen crate.

#![feature(lint_reasons)]
#![warn(
    missing_docs,
    unreachable_pub,
    clippy::pedantic,
    clippy::nursery,
    clippy::undocumented_unsafe_blocks
)]
#![allow(clippy::redundant_pub_crate)] // This would otherwise clash with `unreachable_pub`
#![allow(clippy::module_name_repetitions)]

mod kind;
mod token;

use std::ops::Range;

pub use kind::Kind;
use logos::Logos;
use text_size::{TextRange, TextSize};
pub use token::Token;

/// The central type of this crate, this provides a Lexer, which lexes an input using [`logos`]
///
/// The tokens generated can be accessed through the [`Iterator`] trait.
pub struct Lexer<'a>(logos::Lexer<'a, Kind>);

impl<'a> Lexer<'a> {
    /// Create a new lexer from a source
    ///
    /// # Panics
    ///
    /// This panics if the source string is larger than 4Gb
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        assert!(
            u32::try_from(input.as_bytes().len()).is_ok(),
            "The input has an internal limit of 4Gb per file"
        );

        Self(Kind::lexer(input))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.0.next()?;
        let text = self.0.slice();

        #[allow(
            clippy::cast_possible_truncation,
            reason = "new checks for the possible invariant that we have more than u32 bytes"
        )]
        let range = {
            let Range { start, end } = self.0.span();

            let start = TextSize::from(start as u32);
            let stop = TextSize::from(end as u32);

            TextRange::new(start, stop)
        };

        let token = Token::new(kind, text, range);
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    //! This module tests the regexes not the tokens themselves, as there is no possibility of user
    //! error

    use insta::assert_snapshot;

    use crate::Lexer;

    fn debug(input: &str) -> String {
        let input: Vec<_> = Lexer::new(input).collect();

        format!("{input:?}")
    }

    #[test]
    fn ident() {
        assert_snapshot!("ident", debug("abc"));
        assert_snapshot!("_", debug("_"));
        assert_snapshot!("ident_ident", debug("abc_abc"));
        assert_snapshot!("ident_", debug("ident_"));
    }

    #[test]
    fn string() {
        assert_snapshot!("double_quoted", debug(r#""Double quoted string""#));
        assert_snapshot!(
            "escaped double_quoted",
            debug(r#""Double \\ \" quoted \n \r \x string""#)
        );
        assert_snapshot!("single_quoted", debug(r#"'Single quoted string'"#));
        assert_snapshot!(
            "escaped single_quoted",
            debug(r#"'Single \\ \' quoted \n \r \x string'"#)
        );
    }

    #[test]
    fn integer() {
        assert_snapshot!(debug("12"));
        assert_snapshot!("+", debug("+12"));
        assert_snapshot!("-", debug("-12"));
    }

    #[test]
    fn number() {
        assert_snapshot!(debug("12.0"));
        assert_snapshot!("no_zero", debug("12."));
        assert_snapshot!("no_leading", debug(".12"));
    }

    #[test]
    fn whitespace() {
        assert_snapshot!("tab", debug("	"));
        assert_snapshot!("space", debug(" "));
        assert_snapshot!(
            "newline",
            debug(
                r"
"
            )
        );
    }

    #[test]
    fn comment() {
        assert_snapshot!(debug("// abcdef"));
    }
}
