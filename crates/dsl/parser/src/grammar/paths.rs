//! This is simplified from the original implementation as we do not support any sort of generics

use crate::{
    error::{Expected, ExpectedError},
    grammar::{items, name_ref},
    parser::Parser,
    token_set::TokenSet,
    SyntaxKind,
};

pub(super) const PATH_FIRST: TokenSet = TokenSet::new(&[SyntaxKind::Ident, T![:]]);

pub(crate) fn path(p: &mut Parser) {
    let m = p.start();
    path_segment(p, true);

    let mut qual = m.complete(p, SyntaxKind::Path);

    loop {
        if p.at(T![::]) {
            let m = qual.precede(p);

            p.bump(T![::]);
            path_segment(p, false);

            qual = m.complete(p, SyntaxKind::Path);
        } else {
            break;
        }
    }
}

fn path_segment(p: &mut Parser, first: bool) {
    let m = p.start();

    let mut empty = if first {
        // this is used in the recovery code, we're not empty when we're first because
        // `::std` is valid, while `std::` is not.
        // `::std` would be parsed as: [PATH SEGMENT `std`], because we ignore the first `::`
        p.eat(T![::]);
        false
    } else {
        true
    };

    if p.current() == SyntaxKind::Ident {
        name_ref(p);
    } else {
        p.err_recover(
            ExpectedError::report(p.position(), Expected::Ident),
            items::ITEM_RECOVERY_SET,
        );

        if empty {
            m.abandon(p);
            return;
        }
    }

    m.complete(p, SyntaxKind::PathSegment);
}
