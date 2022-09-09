//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED
//! GENERATED WITH CC7EB731B72484BD593A1459093E405D35546DD67A3DC1451818740512977496

#![allow(missing_doc, reason = "file is automatically generated")]
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
    #[regex("'(?:[^\\']|\\.)*'")]
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
    pub const fn trivia(&self) -> &'static [Self] {
        &[Self::Whitespace, Self::Comment]
    }

    pub fn is_trivia(&self) -> bool {
        self.trivia().contains(self)
    }

    pub const fn literals(&self) -> &'static [Self] {
        &[
            Self::String,
            Self::Integer,
            Self::Number,
            Self::KwTrue,
            Self::KwFalse,
        ]
    }

    pub fn is_literal(&self) -> bool {
        self.literals().contains(self)
    }

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

    pub fn is_infix_op(&self) -> bool {
        self.infix_ops().contains(self)
    }

    pub const fn prefix_ops(&self) -> &'static [Self] {
        &[Self::Plus, Self::Minus]
    }

    pub fn is_prefix_op(&self) -> bool {
        self.prefix_ops().contains(self)
    }

    pub const fn postfix_ops(&self) -> &'static [Self] {
        &[Self::QuestionMark]
    }

    pub fn is_postfix_op(&self) -> bool {
        self.postfix_ops().contains(self)
    }
}
