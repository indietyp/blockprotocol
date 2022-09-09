//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED
//! GENERATED WITH 26454E607F8936A18A9BE13C5FF106D9130E5664F63CA5A6CD221DBA50B1A6ED

#![allow(missing_docs, reason = "file is automatically generated")]
use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};
#[derive(Logos, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum Kind {
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("?")]
    QuestionMark,
    #[token("=")]
    Equals,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("~")]
    Tilde,
    #[token("/")]
    Slash,
    #[token("|")]
    Pipe,
    #[token("@")]
    At,
    #[token("#")]
    Hash,
    #[token("&")]
    Ampersand,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[regex("[_a-zA-Z][a-zA-Z0-9_]*")]
    Ident,
    #[regex("\"(?:[^\\\\\"]|\\\\.)*\"")]
    #[regex("(?x)\n        '(?:[^\\\\']|\\\\.)*'\n    ")]
    String,
    #[regex("[+-]?(([1-9][0-9]*)|0)")]
    Integer,
    #[regex("[+-]?(([1-9][0-9]*)|0)\\.[0-9]+")]
    Number,
    #[token("true")]
    KwTrue,
    #[token("false")]
    KwFalse,
    #[regex("\\s")]
    Whitespace,
    #[regex("//.*")]
    Comment,
    #[error]
    Error,
}
impl Kind {
    #[must_use]
    pub const fn trivia(&self) -> &'static [Self] {
        &[Self::Whitespace, Self::Comment]
    }

    #[must_use]
    pub fn is_trivia(&self) -> bool {
        self.trivia().contains(self)
    }

    #[must_use]
    pub const fn literals(&self) -> &'static [Self] {
        &[
            Self::String,
            Self::Integer,
            Self::Number,
            Self::KwTrue,
            Self::KwFalse,
        ]
    }

    #[must_use]
    pub fn is_literal(&self) -> bool {
        self.literals().contains(self)
    }

    #[must_use]
    pub const fn infix_ops(&self) -> &'static [Self] {
        &[
            Self::Colon,
            Self::Dot,
            Self::Equals,
            Self::GreaterThan,
            Self::LessThan,
            Self::Plus,
            Self::Minus,
            Self::Slash,
            Self::Pipe,
        ]
    }

    #[must_use]
    pub fn is_infix_op(&self) -> bool {
        self.infix_ops().contains(self)
    }

    #[must_use]
    pub const fn prefix_ops(&self) -> &'static [Self] {
        &[Self::Plus, Self::Minus]
    }

    #[must_use]
    pub fn is_prefix_op(&self) -> bool {
        self.prefix_ops().contains(self)
    }

    #[must_use]
    pub const fn postfix_ops(&self) -> &'static [Self] {
        &[Self::QuestionMark]
    }

    #[must_use]
    pub fn is_postfix_op(&self) -> bool {
        self.postfix_ops().contains(self)
    }
}
