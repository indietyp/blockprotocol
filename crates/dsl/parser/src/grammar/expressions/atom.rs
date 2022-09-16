use crate::{
    grammar::{expressions::expr, paths},
    marker::CompletedMarker,
    parser::Parser,
    token_set::TokenSet,
    SyntaxKind,
};

pub(crate) const LITERAL_FIRST: TokenSet = TokenSet::new(&[
    T![true],
    T![false],
    T![null],
    SyntaxKind::Number,
    SyntaxKind::Integer,
    SyntaxKind::String,
]);

// E.g. for after the break in `if break {}`, this should not match
pub(super) const ATOM_EXPR_FIRST: TokenSet = LITERAL_FIRST
    .union(paths::PATH_FIRST)
    .union(TokenSet::new(&[T!['('], T!['{'], T!['[']]));

pub(crate) fn literal_string(p: &mut Parser) -> Option<CompletedMarker> {
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

// TODO: literal prefix and suffix for strings (FIRST)
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

pub(crate) fn tuple_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(T!['(']));

    let m = p.start();
    p.expect(T!['(']);

    let mut saw_comma = false;
    let mut saw_expr = false;

    while !p.at(SyntaxKind::EndOfFile) && !p.at(T![')']) {
        saw_expr = true;

        //> test expr tuple_attrs
        if !expr(p) {
            break;
        }

        if !p.at(T![')']) {
            saw_comma = true;
            p.expect(T![,]);
        }
    }

    p.expect(T![')']);
    m.complete(
        p,
        if saw_expr && !saw_comma {
            SyntaxKind::ParenExpr
        } else {
            SyntaxKind::TupleExpr
        },
    )
}

fn array_expr(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(T!['[']));
    let m = p.start();

    let mut n_exprs = 0u32;
    let mut has_semi = false;

    p.bump(T!['[']);
    while !p.at(SyntaxKind::EndOfFile) && !p.at(T![']']) {
        n_exprs += 1;

        // test array_attrs
        // const A: &[i64] = &[1, #[cfg(test)] 2];
        if !expr(p) {
            break;
        }

        if n_exprs == 1 && p.eat(T![;]) {
            has_semi = true;
            continue;
        }

        if has_semi || !p.at(T![']']) && !p.expect(T![,]) {
            break;
        }
    }
    p.expect(T![']']);

    m.complete(p, SyntaxKind::ListExpr)
}

// TODO: record-expr
// TODO: postfix-expr
// TODO: map-expr
// TODO: reference-ref, name, name-ref
