use crate::{
    error::UnmatchedError,
    grammar::{name, name_ref},
    parser::Parser,
    token_set::TokenSet,
    SyntaxKind,
};

pub(super) const ITEM_RECOVERY_SET: TokenSet = TokenSet::new(&[
    SyntaxKind::DataKw,
    SyntaxKind::PropKw,
    SyntaxKind::PropKw,
    SyntaxKind::LinkKw,
    SyntaxKind::EntityKw,
    SyntaxKind::AliasKw,
    SyntaxKind::UseKw,
    SyntaxKind::SetKw,
    SyntaxKind::ImportKw,
]);

pub(super) fn item(p: &mut Parser) {
    todo!()
}

/// `prop` structurally can be described as:
///
/// ```text
/// prop [id] "string": type (allowed ones) [= default] [;]
/// ```
fn prop(p: &mut Parser) {
    assert!(p.at_contextual_kw(SyntaxKind::DataKw));
    p.bump(SyntaxKind::Ident);

    if p.at(SyntaxKind::Ident) {
        name(p, ITEM_RECOVERY_SET);
    }
}

pub(crate) fn token_tree(p: &mut Parser<'_>) {
    let closing_paren_kind = match p.current() {
        T!['{'] => T!['}'],
        T!['('] => T![')'],
        T!['['] => T![']'],
        _ => unreachable!(),
    };
    let m = p.start();
    p.bump_any();

    while !p.at(SyntaxKind::EndOfFile) && !p.at(closing_paren_kind) {
        match p.current() {
            T!['{'] | T!['('] | T!['['] => token_tree(p),
            // `{` denotes a block, which means they are handled in a special way and we instantly
            // abort
            T!['}'] => {
                p.error(UnmatchedError::report(p.position(), T!['}']));
                m.complete(p, SyntaxKind::TokenTree);
                return;
            }
            token @ (T![')'] | T![']']) => {
                p.err_and_bump(UnmatchedError::report(p.position(), token))
            }
            _ => p.bump_any(),
        }
    }

    p.expect(closing_paren_kind);
    m.complete(p, SyntaxKind::TokenTree);
}
