//! Parser for references, a reference_expr is made up of:
//! `[type] [repo ::] id [/ version]`
use crate::{
    grammar::{name_ref, name_ref_opt},
    marker::CompletedMarker,
    parser::Parser,
    token_set::TokenSet,
    SyntaxKind,
};

fn ref_ty(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at_ts(TokenSet::new(&[
        SyntaxKind::At,
        SyntaxKind::Hash,
        SyntaxKind::GreaterThan,
        SyntaxKind::Tilde,
    ])) {
        let m = p.start();
        p.bump_any();
        Some(m.complete(p, SyntaxKind::ReferenceType))
    } else {
        None
    }
}

fn ref_ident(p: &mut Parser) -> Option<CompletedMarker> {
    let m = match name_ref_opt(p) {
        Some(prev) => prev.precede(p),
        None => {
            return None;
        }
    };

    while p.eat(SyntaxKind::Minus) {
        name_ref(p);
    }

    Some(m.complete(p, SyntaxKind::ReferenceRef))
}

fn ref_version(p: &mut Parser) -> Option<CompletedMarker> {
    if !p.at(SyntaxKind::Slash) {
        return None;
    }

    let m = p.start();

    p.eat(SyntaxKind::Slash);

    if p.at_contextual_kw(SyntaxKind::LatestKw) {
        p.eat(SyntaxKind::Ident);
    } else {
        // TODO: Natural :o
        p.expect(SyntaxKind::Integer);
    }

    Some(m.complete(p, SyntaxKind::ReferenceVersion))
}

fn ref_expr(p: &mut Parser) -> Option<CompletedMarker> {
    todo!()
}
