//! THIS FILE HAS BEEN AUTOMATICALLY GENERATED
//! GENERATED WITH 7C812239C2A95B72F9E2B40DAA712C66A4D286019C4770713D0BBDF37182160E

#![allow(missing_docs, reason = "file is automatically generated")]
use lexer::Kind;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::{
    grammar::{Affix, Associativity, Precedence},
    parser::Parser,
};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive, Copy, Clone, Hash)]
pub enum SyntaxKind {
    Colon,
    Dot,
    QuestionMark,
    ExclamationMark,
    Equals,
    GreaterThan,
    LessThan,
    Plus,
    Minus,
    Star,
    Percent,
    Caret,
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
    DoKw,
    LetKw,
    FnKw,
    LoopKw,
    MatchKw,
    ForKw,
    IfKw,
    ElseKw,
    ReturnKw,
    ContinueKw,
    BreakKw,
    WhileKw,
    TryKw,
    StaticKw,
    MoveKw,
    AsyncKw,
    YieldKw,
    ConstKw,
    Whitespace,
    Comment,
    DataItem,
    PropItem,
    LinkItem,
    EntityItem,
    UseItem,
    SetItem,
    AliasItem,
    AliasDataItem,
    AliasPropItem,
    AliasLinkItem,
    AliasEntityItem,
    RecordExpr,
    RecordExprEntry,
    Literal,
    MapExpr,
    ListExpr,
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
    Path,
    PathSegment,
    LiteralString,
    LiteralStringPrefix,
    LiteralStringSuffix,
    ReferenceRef,
    Name,
    NameRef,
    CommentDoc,
    Attribute,
    Meta,
    TokenTree,
    CallExpr,
    CallArgs,
    Colon2,
    Pipe2,
    Gt2,
    GtEq,
    Lt2,
    LtEq,
    Eq2,
    Star2,
    Ampersand2,
    Dot2,
    Dot2Eq,
    MinusGt,
    TildeGt,
    LtMinus,
    LtTilde,
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
            Kind::ExclamationMark => Self::ExclamationMark,
            Kind::Equals => Self::Equals,
            Kind::GreaterThan => Self::GreaterThan,
            Kind::LessThan => Self::LessThan,
            Kind::Plus => Self::Plus,
            Kind::Minus => Self::Minus,
            Kind::Star => Self::Star,
            Kind::Percent => Self::Percent,
            Kind::Caret => Self::Caret,
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
            Kind::DoKw => Self::DoKw,
            Kind::LetKw => Self::LetKw,
            Kind::FnKw => Self::FnKw,
            Kind::LoopKw => Self::LoopKw,
            Kind::MatchKw => Self::MatchKw,
            Kind::ForKw => Self::ForKw,
            Kind::IfKw => Self::IfKw,
            Kind::ElseKw => Self::ElseKw,
            Kind::ReturnKw => Self::ReturnKw,
            Kind::ContinueKw => Self::ContinueKw,
            Kind::BreakKw => Self::BreakKw,
            Kind::WhileKw => Self::WhileKw,
            Kind::TryKw => Self::TryKw,
            Kind::StaticKw => Self::StaticKw,
            Kind::MoveKw => Self::MoveKw,
            Kind::AsyncKw => Self::AsyncKw,
            Kind::YieldKw => Self::YieldKw,
            Kind::ConstKw => Self::ConstKw,
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
macro_rules ! T { [:] => { $ crate :: SyntaxKind :: Colon } ; [.] => { $ crate :: SyntaxKind :: Dot } ; [?] => { $ crate :: SyntaxKind :: QuestionMark } ; [!] => { $ crate :: SyntaxKind :: ExclamationMark } ; [=] => { $ crate :: SyntaxKind :: Equals } ; [>] => { $ crate :: SyntaxKind :: GreaterThan } ; [<] => { $ crate :: SyntaxKind :: LessThan } ; [+] => { $ crate :: SyntaxKind :: Plus } ; [-] => { $ crate :: SyntaxKind :: Minus } ; [*] => { $ crate :: SyntaxKind :: Star } ; [%] => { $ crate :: SyntaxKind :: Percent } ; [^] => { $ crate :: SyntaxKind :: Caret } ; [~] => { $ crate :: SyntaxKind :: Tilde } ; [/] => { $ crate :: SyntaxKind :: Slash } ; [|] => { $ crate :: SyntaxKind :: Pipe } ; [@] => { $ crate :: SyntaxKind :: At } ; [#] => { $ crate :: SyntaxKind :: Hash } ; [&] => { $ crate :: SyntaxKind :: Ampersand } ; [;] => { $ crate :: SyntaxKind :: Semicolon } ; [,] => { $ crate :: SyntaxKind :: Comma } ; ['}'] => { $ crate :: SyntaxKind :: LBrace } ; ['{'] => { $ crate :: SyntaxKind :: RBrace } ; ['('] => { $ crate :: SyntaxKind :: LParen } ; [')'] => { $ crate :: SyntaxKind :: RParen } ; ['['] => { $ crate :: SyntaxKind :: LBracket } ; [']'] => { $ crate :: SyntaxKind :: RBracket } ; [true] => { $ crate :: SyntaxKind :: TrueKw } ; [false] => { $ crate :: SyntaxKind :: FalseKw } ; [null] => { $ crate :: SyntaxKind :: NullKw } ; [do] => { $ crate :: SyntaxKind :: DoKw } ; [let] => { $ crate :: SyntaxKind :: LetKw } ; [fn] => { $ crate :: SyntaxKind :: FnKw } ; [loop] => { $ crate :: SyntaxKind :: LoopKw } ; [match] => { $ crate :: SyntaxKind :: MatchKw } ; [for] => { $ crate :: SyntaxKind :: ForKw } ; [if] => { $ crate :: SyntaxKind :: IfKw } ; [else] => { $ crate :: SyntaxKind :: ElseKw } ; [return] => { $ crate :: SyntaxKind :: ReturnKw } ; [continue] => { $ crate :: SyntaxKind :: ContinueKw } ; [break] => { $ crate :: SyntaxKind :: BreakKw } ; [while] => { $ crate :: SyntaxKind :: WhileKw } ; [try] => { $ crate :: SyntaxKind :: TryKw } ; [static] => { $ crate :: SyntaxKind :: StaticKw } ; [move] => { $ crate :: SyntaxKind :: MoveKw } ; [async] => { $ crate :: SyntaxKind :: AsyncKw } ; [yield] => { $ crate :: SyntaxKind :: YieldKw } ; [const] => { $ crate :: SyntaxKind :: ConstKw } ; [::] => { $ crate :: SyntaxKind :: Colon2 } ; [||] => { $ crate :: SyntaxKind :: Pipe2 } ; [>>] => { $ crate :: SyntaxKind :: Gt2 } ; [>=] => { $ crate :: SyntaxKind :: GtEq } ; [<<] => { $ crate :: SyntaxKind :: Lt2 } ; [<=] => { $ crate :: SyntaxKind :: LtEq } ; [==] => { $ crate :: SyntaxKind :: Eq2 } ; [**] => { $ crate :: SyntaxKind :: Star2 } ; [&&] => { $ crate :: SyntaxKind :: Ampersand2 } ; [..] => { $ crate :: SyntaxKind :: Dot2 } ; [..=] => { $ crate :: SyntaxKind :: Dot2Eq } ; [->] => { $ crate :: SyntaxKind :: MinusGt } ; [~>] => { $ crate :: SyntaxKind :: TildeGt } ; [<-] => { $ crate :: SyntaxKind :: LtMinus } ; [<~] => { $ crate :: SyntaxKind :: LtTilde } ; }
impl SyntaxKind {
    pub(crate) fn n_raw_tokens(&self) -> u8 {
        match self {
            SyntaxKind::Colon2 => 2u8,
            SyntaxKind::Pipe2 => 2u8,
            SyntaxKind::Gt2 => 2u8,
            SyntaxKind::GtEq => 2u8,
            SyntaxKind::Lt2 => 2u8,
            SyntaxKind::LtEq => 2u8,
            SyntaxKind::Eq2 => 2u8,
            SyntaxKind::Star2 => 2u8,
            SyntaxKind::Ampersand2 => 2u8,
            SyntaxKind::Dot2 => 2u8,
            SyntaxKind::Dot2Eq => 3u8,
            SyntaxKind::MinusGt => 2u8,
            SyntaxKind::TildeGt => 2u8,
            SyntaxKind::LtMinus => 2u8,
            SyntaxKind::LtTilde => 2u8,
            _ => 1,
        }
    }
}
impl Parser<'_> {
    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::Colon2 => self.at_composite2(n, T ! [:], T ! [:]),
            SyntaxKind::Pipe2 => self.at_composite2(n, T ! [|], T ! [|]),
            SyntaxKind::Gt2 => self.at_composite2(n, T ! [>], T ! [>]),
            SyntaxKind::GtEq => self.at_composite2(n, T ! [>], T ! [=]),
            SyntaxKind::Lt2 => self.at_composite2(n, T ! [<], T ! [<]),
            SyntaxKind::LtEq => self.at_composite2(n, T ! [<], T ! [=]),
            SyntaxKind::Eq2 => self.at_composite2(n, T ! [=], T ! [=]),
            SyntaxKind::Star2 => self.at_composite2(n, T ! [*], T ! [*]),
            SyntaxKind::Ampersand2 => self.at_composite2(n, T ! [&], T ! [&]),
            SyntaxKind::Dot2 => self.at_composite2(n, T ! [.], T ! [.]),
            SyntaxKind::Dot2Eq => self.at_composite3(n, T ! [.], T ! [.], T ! [=]),
            SyntaxKind::MinusGt => self.at_composite2(n, T ! [-], T ! [>]),
            SyntaxKind::TildeGt => self.at_composite2(n, T ! [~], T ! [>]),
            SyntaxKind::LtMinus => self.at_composite2(n, T ! [<], T ! [-]),
            SyntaxKind::LtTilde => self.at_composite2(n, T ! [<], T ! [~]),
            _ => self.inp.kind(self.pos + n) == kind,
        }
    }
}
impl Affix {
    pub(crate) fn infix(p: &Parser) -> Option<(SyntaxKind, Associativity, Precedence)> {
        if p.at(T ! [..=]) {
            Some((T ! [..=], Associativity::Left, Precedence(3u32)))
        } else if p.at(T![..]) {
            Some((T![..], Associativity::Left, Precedence(3u32)))
        } else if p.at(T ! [||]) {
            Some((T ! [||], Associativity::Left, Precedence(5u32)))
        } else if p.at(T ! [&&]) {
            Some((T ! [&&], Associativity::Left, Precedence(7u32)))
        } else if p.at(T ! [==]) {
            Some((T ! [==], Associativity::Left, Precedence(11u32)))
        } else if p.at(T ! [<=]) {
            Some((T ! [<=], Associativity::Left, Precedence(11u32)))
        } else if p.at(T ! [>=]) {
            Some((T ! [>=], Associativity::Left, Precedence(11u32)))
        } else if p.at(T ! [>>]) {
            Some((T ! [>>], Associativity::Left, Precedence(13u32)))
        } else if p.at(T ! [<<]) {
            Some((T ! [<<], Associativity::Left, Precedence(13u32)))
        } else if p.at(T ! [**]) {
            Some((T ! [**], Associativity::Right, Precedence(25u32)))
        } else if p.at(T ! [=]) {
            Some((T ! [=], Associativity::Left, Precedence(1u32)))
        } else if p.at(T ! [<]) {
            Some((T ! [<], Associativity::Left, Precedence(11u32)))
        } else if p.at(T ! [>]) {
            Some((T ! [>], Associativity::Left, Precedence(11u32)))
        } else if p.at(T ! [^]) {
            Some((T ! [^], Associativity::Left, Precedence(15u32)))
        } else if p.at(T ! [&]) {
            Some((T ! [&], Associativity::Left, Precedence(17u32)))
        } else if p.at(T ! [|]) {
            Some((T ! [|], Associativity::Left, Precedence(19u32)))
        } else if p.at(T ! [+]) {
            Some((T ! [+], Associativity::Left, Precedence(21u32)))
        } else if p.at(T ! [-]) {
            Some((T ! [-], Associativity::Left, Precedence(21u32)))
        } else if p.at(T ! [*]) {
            Some((T ! [*], Associativity::Left, Precedence(23u32)))
        } else if p.at(T ! [/]) {
            Some((T ! [/], Associativity::Left, Precedence(23u32)))
        } else if p.at(T ! [%]) {
            Some((T ! [%], Associativity::Left, Precedence(23u32)))
        } else {
            None
        }
    }

    pub(crate) fn prefix(p: &Parser) -> Option<(SyntaxKind, Precedence)> {
        if p.at(T![!]) {
            Some((T![!], Precedence(9u32)))
        } else if p.at(T ! [+]) {
            Some((T ! [+], Precedence(27u32)))
        } else if p.at(T ! [-]) {
            Some((T ! [-], Precedence(27u32)))
        } else {
            None
        }
    }

    pub(crate) fn postfix(p: &Parser) -> Option<(SyntaxKind, Precedence)> {
        if p.at(T ! [?]) {
            Some((T ! [?], Precedence(29u32)))
        } else {
            None
        }
    }
}
