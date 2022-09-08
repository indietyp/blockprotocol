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
}
