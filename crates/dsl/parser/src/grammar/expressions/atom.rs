use crate::{marker::CompletedMarker, parser::Parser, token_set::TokenSet, SyntaxKind};

pub(crate) const LITERAL_FIRST: TokenSet = TokenSet::new(&[
    T![true],
    T![false],
    T![null],
    SyntaxKind::Number,
    SyntaxKind::Integer,
    SyntaxKind::String,
]);

fn literal_string(p: &mut Parser) -> Option<CompletedMarker> {
    if p.nth_at(0, SyntaxKind::Ident)
        && p.nth_at(1, SyntaxKind::String)
        && p.nth_at(2, SyntaxKind::Ident)
    {
        // a"example"b
        let m = p.start();

        let prefix = p.start();
        p.bump(SyntaxKind::Ident);
        prefix.complete(p, SyntaxKind::LiteralStringPrefix);

        p.bump(SyntaxKind::String);

        let suffix = p.start();
        p.bump(SyntaxKind::Ident);
        suffix.complete(p, SyntaxKind::LiteralStringSuffix);

        Some(m.complete(p, SyntaxKind::LiteralString))
    } else if p.nth_at(0, SyntaxKind::Ident) && p.nth_at(1, SyntaxKind::String) {
        // a"example"
        let m = p.start();

        let prefix = p.start();
        p.bump(SyntaxKind::Ident);
        prefix.complete(p, SyntaxKind::LiteralStringPrefix);

        p.bump(SyntaxKind::String);

        Some(m.complete(p, SyntaxKind::LiteralString))
    } else if p.nth_at(0, SyntaxKind::String) && p.nth_at(1, SyntaxKind::Ident) {
        // "example"b

        let m = p.start();

        p.bump(SyntaxKind::String);

        let suffix = p.start();
        p.bump(SyntaxKind::Ident);
        suffix.complete(p, SyntaxKind::LiteralStringSuffix);

        Some(m.complete(p, SyntaxKind::LiteralString))
    } else if p.at(SyntaxKind::String) {
        // "example"

        let m = p.start();

        p.bump(SyntaxKind::String);

        Some(m.complete(p, SyntaxKind::LiteralString))
    } else {
        None
    }
}

// TODO: literal prefix and suffix for strings
pub(crate) fn literal(p: &mut Parser) -> Option<CompletedMarker> {
    if let Some(marker) = literal_string(p) {
        return Some(marker);
    }

    if !p.at_ts(LITERAL_FIRST) {
        return None;
    }

    let m = p.start();
    p.bump_any();
    Some(m.complete(p, SyntaxKind::Literal))
}
