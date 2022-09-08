//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED
//! GENERATED WITH AF179FDBFDC76D11CFD8B9E09BDCEB12A26E23EA8A9CBC5D31B2C804B5B73999

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
    Lbrace,
    #[token("}")]
    Rbrace,
    #[token("(")]
    Lparen,
    #[token(")")]
    Rparen,
    #[token("[")]
    Lbracket,
    #[token("]")]
    Rbracket,
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
}
