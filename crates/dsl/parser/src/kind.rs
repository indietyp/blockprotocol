//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED
//! GENERATED WITH 026877BED57CE5769FA9448A3324F08A82F5841809A625ED58D076AA096832C0

#![allow(missing_docs, reason = "file is automatically generated")]
use lexer::Kind;
use num_derive::{FromPrimitive, ToPrimitive};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive, Copy, Clone, Hash)]
pub enum SyntaxKind {
    Colon,
    Dot,
    QuestionMark,
    Equals,
    GreaterThan,
    LessThan,
    Plus,
    Minus,
    Star,
    Tilde,
    Slash,
    Pipe,
    At,
    Hash,
    Ampersand,
    Semicolon,
    Comma,
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Ident,
    String,
    Integer,
    Number,
    TrueKw,
    FalseKw,
    NullKw,
    LetKw,
    FnKw,
    Whitespace,
    Comment,
    DataStmt,
    PropStmt,
    LinkStmt,
    EntityStmt,
    LetStmt,
    UseStmt,
    SetStmt,
    AliasDataStmt,
    AliasPropStmt,
    AliasLinkStmt,
    AliasEntityStmt,
    RecordExpr,
    RecordExprEntry,
    LiteralExpr,
    StringExpr,
    MapExpr,
    ListExpr,
    ModuleExpr,
    ParenExpr,
    InfixExpr,
    PrefixExpr,
    PostfixExpr,
    TupleExpr,
    RangeExpr,
    UnionType,
    RecordType,
    RecordTypeEntry,
    ListType,
    MapType,
    TupleType,
    ReferenceIdent,
    Reference,
    Variable,
    CommentDoc,
    Attribute,
    CallExpr,
    CallArgs,
    Colon2,
    Dot2,
    Dot2Eq,
    DataKw,
    PropKw,
    LinkKw,
    EntityKw,
    AliasKw,
    UseKw,
    SetKw,
    EndOfFile,
    Error,
    Tombstone,
}
impl From<Kind> for SyntaxKind {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Colon => Self::Colon,
            Kind::Dot => Self::Dot,
            Kind::QuestionMark => Self::QuestionMark,
            Kind::Equals => Self::Equals,
            Kind::GreaterThan => Self::GreaterThan,
            Kind::LessThan => Self::LessThan,
            Kind::Plus => Self::Plus,
            Kind::Minus => Self::Minus,
            Kind::Star => Self::Star,
            Kind::Tilde => Self::Tilde,
            Kind::Slash => Self::Slash,
            Kind::Pipe => Self::Pipe,
            Kind::At => Self::At,
            Kind::Hash => Self::Hash,
            Kind::Ampersand => Self::Ampersand,
            Kind::Semicolon => Self::Semicolon,
            Kind::Comma => Self::Comma,
            Kind::LBrace => Self::LBrace,
            Kind::RBrace => Self::RBrace,
            Kind::LParen => Self::LParen,
            Kind::RParen => Self::RParen,
            Kind::LBracket => Self::LBracket,
            Kind::RBracket => Self::RBracket,
            Kind::Ident => Self::Ident,
            Kind::String => Self::String,
            Kind::Integer => Self::Integer,
            Kind::Number => Self::Number,
            Kind::TrueKw => Self::TrueKw,
            Kind::FalseKw => Self::FalseKw,
            Kind::NullKw => Self::NullKw,
            Kind::LetKw => Self::LetKw,
            Kind::FnKw => Self::FnKw,
            Kind::Whitespace => Self::Whitespace,
            Kind::Comment => Self::Comment,
            Kind::Error => Self::Error,
        }
    }
}
impl SyntaxKind {
    #[must_use]
    pub const fn trivia(&self) -> &'static [Self] {
        &[Self::Whitespace, Self::Comment]
    }

    #[must_use]
    pub fn is_trivia(&self) -> bool {
        self.trivia().contains(self)
    }

    pub fn from_contextual_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "data" => Self::DataKw,
            "prop" => Self::PropKw,
            "link" => Self::LinkKw,
            "entity" => Self::EntityKw,
            "alias" => Self::AliasKw,
            "use" => Self::UseKw,
            "set" => Self::SetKw,
            _ => return None,
        };
        Some(kw)
    }
}
macro_rules ! T  { [:] => { $ crate :: SyntaxKind :: Colon } ; [.] => { $ crate :: SyntaxKind :: Dot } ; [?] => { $ crate :: SyntaxKind :: QuestionMark } ; [=] => { $ crate :: SyntaxKind :: Equals } ; [>] => { $ crate :: SyntaxKind :: GreaterThan } ; [<] => { $ crate :: SyntaxKind :: LessThan } ; [+] => { $ crate :: SyntaxKind :: Plus } ; [-] => { $ crate :: SyntaxKind :: Minus } ; [*] => { $ crate :: SyntaxKind :: Star } ; [~] => { $ crate :: SyntaxKind :: Tilde } ; [/] => { $ crate :: SyntaxKind :: Slash } ; [|] => { $ crate :: SyntaxKind :: Pipe } ; [@] => { $ crate :: SyntaxKind :: At } ; [#] => { $ crate :: SyntaxKind :: Hash } ; [&] => { $ crate :: SyntaxKind :: Ampersand } ; [;] => { $ crate :: SyntaxKind :: Semicolon } ; [,] => { $ crate :: SyntaxKind :: Comma } ; ['}'] => { $ crate :: SyntaxKind :: LBrace } ; ['{'] => { $ crate :: SyntaxKind :: RBrace } ; ['('] => { $ crate :: SyntaxKind :: LParen } ; [')'] => { $ crate :: SyntaxKind :: RParen } ; ['['] => { $ crate :: SyntaxKind :: LBracket } ; [']'] => { $ crate :: SyntaxKind :: RBracket } ; [true] => { $ crate :: SyntaxKind :: TrueKw } ; [false] => { $ crate :: SyntaxKind :: FalseKw } ; [null] => { $ crate :: SyntaxKind :: NullKw } ; [let] => { $ crate :: SyntaxKind :: LetKw } ; [fn] => { $ crate :: SyntaxKind :: FnKw } ; [::] => { $ crate :: SyntaxKind :: Colon2 } ; [..] => { $ crate :: SyntaxKind :: Dot2 } ; [..=] => { $ crate :: SyntaxKind :: Dot2Eq } ; }
