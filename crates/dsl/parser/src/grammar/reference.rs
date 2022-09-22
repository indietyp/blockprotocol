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

    while p.eat(T![-]) {
        name_ref(p);
    }

    Some(m.complete(p, SyntaxKind::ReferenceRef))
}

fn ref_version(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    if p.at_contextual_kw(SyntaxKind::LatestKw) {
        p.eat(SyntaxKind::Ident);
    } else {
        p.expect(SyntaxKind::Integer);
    }

    m.complete(p, SyntaxKind::ReferenceVersion)
}

pub(in crate::grammar) fn might_be_ref(p: &Parser) -> bool {
    p.at_ts(TokenSet::new(&[
        T![@],
        T![#],
        T![>],
        T![~],
        SyntaxKind::Ident,
    ]))
}

pub(in crate::grammar) fn ref_expr(p: &mut Parser) -> Option<CompletedMarker> {
    if !might_be_ref(p) {
        return None;
    }

    let m = p.start();

    ref_ty(p);

    // conditional parsing, depending on if we have `::` the ident is either id or repo.
    // we use `precede()` to surround it with the correct kind.
    let repo_or_id = ref_ident(p);

    if p.eat(T![::]) {
        if let Some(repo) = repo_or_id {
            let m = repo.precede(p);
            m.complete(p, SyntaxKind::ReferenceRepo);
        }

        let m = p.start();
        ref_ident(p);

        m.complete(p, SyntaxKind::ReferenceId);
    } else if let Some(id) = repo_or_id {
        let m = id.precede(p);
        m.complete(p, SyntaxKind::ReferenceId);
    }

    if p.eat(T![/]) {
        ref_version(p);
    }

    Some(m.complete(p, SyntaxKind::ReferenceRef))
}
