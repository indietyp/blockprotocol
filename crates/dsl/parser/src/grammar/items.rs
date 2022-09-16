use crate::{
    error::{Expected, ExpectedError, UnmatchedError},
    grammar::{expressions::atom::literal_string, name, name_ref, paths::path},
    marker::Marker,
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
]);

pub(super) fn item(p: &mut Parser) {
    todo!()
}

/// `prop` item
///
/// ```text
/// prop [id] "string": type (allowed ones) [= default] ;
/// ```
fn prop(p: &mut Parser, m: Marker) {
    assert!(p.at_contextual_kw(SyntaxKind::DataKw));
    p.bump(SyntaxKind::Ident);

    if p.at(SyntaxKind::Ident) {
        name(p, ITEM_RECOVERY_SET);
    }

    if literal_string(p).is_none() {
        p.err_recover(
            ExpectedError::report(p.position(), Expected::String),
            ITEM_RECOVERY_SET,
        );
    }

    p.expect(T![:]);

    // TODO: types

    if p.eat(T![=]) {
        // TODO: default
    }

    p.expect(T![;]);
    m.complete(p, SyntaxKind::PropItem);
}

/// `entity` item
///
/// ```text
/// entity [id] "string": type (record w/ links) [= default] ;
/// ```
fn entity(p: &mut Parser, m: Marker) {
    assert!(p.at_contextual_kw(SyntaxKind::EntityKw));
    p.bump(SyntaxKind::Ident);

    if p.at(SyntaxKind::Ident) {
        name(p, ITEM_RECOVERY_SET);
    }

    if literal_string(p).is_none() {
        p.err_recover(
            ExpectedError::report(p.position(), Expected::String),
            ITEM_RECOVERY_SET,
        )
    }

    p.expect(T![:]);

    // TODO: type

    if p.eat(T![=]) {
        // TODO: default
    }

    p.expect(T![;]);
    m.complete(p, SyntaxKind::EntityItem);
}

/// `data` item
///
/// ```text
/// data [id] "string": type (tbd) ;
/// ```
///
/// TODO: insert id once it has been run once and an id as been assigned
fn data(p: &mut Parser, m: Marker) {
    assert!(p.at_contextual_kw(SyntaxKind::DataKw));
    p.bump(SyntaxKind::Ident);

    if p.at(SyntaxKind::Ident) {
        name(p, ITEM_RECOVERY_SET);
    }

    if literal_string(p).is_none() {
        p.err_recover(
            ExpectedError::report(p.position(), Expected::String),
            ITEM_RECOVERY_SET,
        );
    }

    p.expect(T![:]);

    // TODO: type

    p.expect(T![;]);
    m.complete(p, SyntaxKind::DataItem);
}

/// `link` item
///
/// ```text
/// link [id] "string" ;
/// ```
fn link(p: &mut Parser, m: Marker) {
    assert!(p.at_contextual_kw(SyntaxKind::LinkKw));
    p.bump(SyntaxKind::Ident);

    if p.at(SyntaxKind::Ident) {
        name(p, ITEM_RECOVERY_SET);
    }

    if literal_string(p).is_none() {
        p.err_recover(
            ExpectedError::report(p.position(), Expected::String),
            ITEM_RECOVERY_SET,
        );
    }

    p.expect(T![;]);
    m.complete(p, SyntaxKind::LinkItem);
}

/// `use` item
///
/// ```text
/// use path ;
/// ```
fn use_(p: &mut Parser, m: Marker) {
    assert!(p.at_contextual_kw(SyntaxKind::UseKw));
    p.bump(SyntaxKind::Ident);

    path(p);

    p.expect(T![;]);
    m.complete(p, SyntaxKind::UseItem);
}

/// `set` item
///
/// ```set
/// set path = expr ;
/// ```
fn set(p: &mut Parser, m: Marker) {
    assert!(p.at_contextual_kw(SyntaxKind::SetKw));
    p.bump(SyntaxKind::Ident);

    path(p);

    p.expect(SyntaxKind::Equals);

    // TODO: expression

    p.expect(T![;]);
    m.complete(p, SyntaxKind::SetItem);
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
