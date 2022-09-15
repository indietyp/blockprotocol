use crate::{marker::CompletedMarker, parser::Parser, token_set::TokenSet, SyntaxKind};

pub(crate) const LITERAL_FIRST: TokenSet = TokenSet::new(&[
    T![true],
    T![false],
    T![null],
    SyntaxKind::Number,
    SyntaxKind::Integer,
    SyntaxKind::String,
]);

// TODO: literal prefix and suffix for strings
pub(crate) fn literal(p: &mut Parser) -> Option<CompletedMarker> {
    if !p.at_ts(LITERAL_FIRST) {
        return None;
    }

    let m = p.start();
    p.bump_any();
    Some(m.complete(p, SyntaxKind::Literal))
}
