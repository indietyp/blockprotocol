//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED
//! GENERATED WITH 10195B2C50AE08E017069DE204BDEDCB4CFF3654DA7CAF43734FF17A287658DE

#![allow(missing_docs, reason = "file is automatically generated")]
use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};
#[derive(
    Logos, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive, Copy, Clone, Hash,
)]
pub enum Kind {
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("?")]
    QuestionMark,
    #[token("!")]
    ExclamationMark,
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
    #[token("%")]
    Percent,
    #[token("^")]
    Caret,
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
    #[regex("(([1-9][0-9]*)|0)")]
    Integer,
    #[regex("[+-]?(([1-9][0-9]*)|0)\\.[0-9]+")]
    Number,
    #[token("true")]
    TrueKw,
    #[token("false")]
    FalseKw,
    #[token("null")]
    NullKw,
    #[token("do")]
    DoKw,
    #[token("let")]
    LetKw,
    #[token("fn")]
    FnKw,
    #[token("loop")]
    LoopKw,
    #[token("match")]
    MatchKw,
    #[token("for")]
    ForKw,
    #[token("if")]
    IfKw,
    #[token("else")]
    ElseKw,
    #[token("return")]
    ReturnKw,
    #[token("continue")]
    ContinueKw,
    #[token("break")]
    BreakKw,
    #[token("while")]
    WhileKw,
    #[token("try")]
    TryKw,
    #[token("static")]
    StaticKw,
    #[token("move")]
    MoveKw,
    #[token("async")]
    AsyncKw,
    #[token("yield")]
    YieldKw,
    #[token("const")]
    ConstKw,
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
            Self::TrueKw,
            Self::FalseKw,
            Self::NullKw,
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
        &[Self::ExclamationMark, Self::Plus, Self::Minus]
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
