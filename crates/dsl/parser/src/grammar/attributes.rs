//! Adapted from:
//! https://github.com/rust-lang/rust-analyzer/blob/6b163c301f70d0e1246fb898b5f5edcc4d03fa4c/crates/parser/src/grammar/attributes.rs

use crate::{
    grammar::{items, paths},
    parser::Parser,
    SyntaxKind,
};

pub(super) fn inner_attrs(p: &mut Parser<'_>) {
    while p.at(T![#]) && p.nth(1) == T![!] {
        attr(p, true);
    }
}

pub(super) fn outer_attrs(p: &mut Parser<'_>) {
    while p.at(T![#]) {
        attr(p, false);
    }
}

fn attr(p: &mut Parser<'_>, inner: bool) {
    assert!(p.at(T![#]));

    let attr = p.start();
    p.bump(T![#]);

    if inner {
        p.bump(T![!]);
    }

    if p.expect(T!['[']) {
        meta(p);

        p.expect(T![']']);
    }

    attr.complete(p, SyntaxKind::Attribute);
}

pub(super) fn meta(p: &mut Parser<'_>) {
    let meta = p.start();
    paths::path(p);

    match p.current() {
        T![=] => {
            p.bump(T![=]);
            // TODO
            if !expressions::expr(p) {
                p.error("expected expression");
            }
        }
        T!['('] | T!['['] | T!['{'] => items::token_tree(p),
        _ => {}
    }

    meta.complete(p, SyntaxKind::Meta);
}
