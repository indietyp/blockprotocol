use crate::{grammar::BlockLike, marker::CompletedMarker, parser::Parser};

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
            T![.] => match postfix_dot_expr(p, lhs) {
                Ok(it) => it,
                Err(it) => {
                    lhs = it;
                    break;
                }
            },
            _ => break,
        };

        allow_calls = true;
        block_like = BlockLike::NotBlock
    }

    (lhs, block_like)
}
