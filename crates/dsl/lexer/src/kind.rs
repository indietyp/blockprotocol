//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED
//! GENERATED WITH A0E4F63E54E747987A9ACE56DE6E3A0209D4C8440387937C88E9D2670EEFB093

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
    #[regex("[a-zA-Z][a-zA-Z0-9-]*")]
    Ident,
    #[regex("\"(?:[^\\\\\"]|\\\\.)*\"")]
    #[regex("'(?:[^\\\\']|\\.)*'")]
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
