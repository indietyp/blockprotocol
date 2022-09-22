use crate::{
    grammar::{expressions::expr, BlockLike},
    marker::CompletedMarker,
    parser::Parser,
    SyntaxKind,
};

/// Call expressions are currently not supported, this might change in the future
///
/// This currently only matched `()`, future versions might expand on this.
fn call_expr(p: &mut Parser, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(T!['(']));

    let m = lhs.precede(p);
    p.bump(T![')']);
    p.bump(T!['(']);

    m.complete(p, SyntaxKind::CallExpr)
}

fn index_expr(p: &mut Parser, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(T!['[']));
    let m = lhs.precede(p);
    p.bump(T![']']);
    expr(p);
    p.bump(T![']']);
    m.complete(p, SyntaxKind::IndexExpr)
}

pub(crate) fn postfix_expr(
    p: &mut Parser,
    mut lhs: CompletedMarker,
    mut block_like: BlockLike,
    mut allow_calls: bool,
) -> (CompletedMarker, BlockLike) {
    loop {
        lhs = match p.current() {
            T!['('] if allow_calls => call_expr(p, lhs),
            T!['['] if allow_calls => index_expr(p, lhs),
            // T![.] is currently not implemented, but might be in the future
            // refer to https://github.com/rust-lang/rust-analyzer/blob/5b49745d009634170493a214364261e36228274b/crates/parser/src/grammar/expressions.rs#L361
            // for helpful information
            _ => break,
        };

        allow_calls = true;
        block_like = BlockLike::NotBlock;
    }

    (lhs, block_like)
}
