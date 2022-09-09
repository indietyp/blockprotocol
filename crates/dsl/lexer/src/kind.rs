//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED
//! GENERATED WITH 17E144E83C1B3F5FEB534FFA86E9FF752AC60565B857CC37FFDD842722CA13AD

use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};
#[derive(Logos, Debug, PartialEq, ToPrimitive, Copy, Clone)]
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
    pub fn trivia(&self) -> &'static [Self] {
        &[]
    }

    pub fn is_trivia(&self) -> bool {
        self.trivia().contains(self)
    }

    pub fn literals(&self) -> &'static [Self] {
        &[]
    }

    pub fn is_literals(&self) -> bool {
        self.literal().contains(self)
    }

    pub fn infix_ops(&self) -> &'static [Self] {
        &[]
    }

    pub fn is_infix_op(&self) -> bool {
        self.infix_ops().contains(self)
    }

    pub fn prefix_ops(&self) -> &'static [Self] {
        &[]
    }

    pub fn is_prefix_op(&self) -> bool {
        self.prefix_ops().contains(self)
    }

    pub fn suffix_ops(&self) -> &'static [Self] {
        &[]
    }

    pub fn is_suffix_op(&self) -> bool {
        self.suffix_ops().contains(self)
    }
}
