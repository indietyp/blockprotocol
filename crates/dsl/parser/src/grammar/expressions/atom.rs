use crate::{
    error::{Expected, ExpectedError},
    grammar::{
        attributes,
        expressions::{expr, reference::ref_expr, Restrictions},
        paths, BlockLike,
    },
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

// for now unused
const EXPR_RECOVERY_SET: TokenSet = TokenSet::new(&[T![let]]);

pub(crate) fn literal_string(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at_composite3(0, SyntaxKind::Ident, SyntaxKind::String, SyntaxKind::Ident) {
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
    } else if p.at_composite2(0, SyntaxKind::Ident, SyntaxKind::String) {
        // a"example"
        let m = p.start();

        let prefix = p.start();
        p.bump(SyntaxKind::Ident);
        prefix.complete(p, SyntaxKind::LiteralStringPrefix);

        p.bump(SyntaxKind::String);

        Some(m.complete(p, SyntaxKind::LiteralString))
    } else if p.at_composite2(0, SyntaxKind::String, SyntaxKind::Ident) {
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

fn list_expr(p: &mut Parser<'_>) -> CompletedMarker {
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

/// This is currently not implemented, and only matches `do { }`
///
/// This is reserved syntax, which is parsed, but not syntax and may be used in the future.
fn block_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(T![do]));

    let m = p.start();
    p.expect(T![do]);
    p.expect(T!['{']);
    p.expect(T!['}']);

    m.complete(p, SyntaxKind::BlockExpr)
}

fn map_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(T!['{']));

    let m = p.start();
    p.bump(T!['{']);

    while !p.eof() && !p.at(T!['}']) {
        let m = p.start();

        attributes::outer_attrs(p);

        if ref_expr(p).is_some() {
            p.expect(T![:]);
            expr(p);
            m.complete(p, SyntaxKind::MapExprField);
        } else {
            p.err_and_bump(ExpectedError::report(p.pos, Expected::Reference));
            m.abandon(p);
        }

        if !p.at(T!['}']) {
            p.expect(T![,]);
        }
    }

    m.complete(p, SyntaxKind::MapExpr)
}

fn path_expr(p: &mut Parser) -> (CompletedMarker, BlockLike) {
    assert!(p.at(SyntaxKind::Ident));

    let m = p.start();
    paths::path(p);

    match p.current() {
        T!['{'] => {
            map_expr(p);
            (m.complete(p, SyntaxKind::RecordExpr), BlockLike::NotBlock)
        }
        _ => (m.complete(p, SyntaxKind::PathExpr), BlockLike::NotBlock),
    }
}

// TODO: name, name-ref
pub(crate) fn atom_expr(p: &mut Parser, r: Restrictions) -> Option<(CompletedMarker, BlockLike)> {
    if let Some(m) = literal(p) {
        return Some((m, BlockLike::NotBlock));
    }

    if p.at(SyntaxKind::Ident) {
        return Some(path_expr(p));
    }

    let done = match p.current() {
        T!['('] => tuple_expr(p),
        T!['['] => list_expr(p),
        T![do] => block_expr(p),
        T!['{'] => map_expr(p),
        _ => {
            p.err_recover(
                ExpectedError::report(p.pos, Expected::Expr),
                EXPR_RECOVERY_SET,
            );

            return None;
        }
    };

    let block_like = match done.kind() {
        SyntaxKind::BlockExpr => BlockLike::Block,
        _ => BlockLike::NotBlock,
    };

    Some((done, block_like))
}
