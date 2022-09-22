//! Types in Pika deviate quite a bit from rust types.
//!
//! Types that can be defined in Pika are:
//!     * List (variable or fixed size)
//!     * Anonymous Record
//!     * Map (TBD)
//!     * Tuple (TBD)
//!     * Union
//!     * Reference
//!
//! Depending on the type declared there is only a subset available.
//!
//! Entity-Types:
//!     * Anonymous Record
//!
//! Property-Types:
//!     * List<..>
//!     * Reference (Data-Type)
//!     * Anonymous Record
//!     * Union<..>
//!
//! Data-Type:
//!     * Anonymous Record, but only id as key

use crate::{
    error::{Expected, ExpectedError},
    grammar::{attributes, expressions::expr, reference::ref_expr},
    marker::CompletedMarker,
    parser::Parser,
    SyntaxKind,
};

fn list_type_range(p: &mut Parser) {
    let m = p.start();

    if p.nth_at(0, SyntaxKind::Integer) && p.nth_at(1, T![..]) {
        let min = p.start();
        p.bump(SyntaxKind::Integer);
        min.complete(p, SyntaxKind::ListTypeRangeMin);

        p.bump(T![..]);

        if p.at(SyntaxKind::Integer) {
            let max = p.start();
            p.bump(SyntaxKind::Integer);
            max.complete(p, SyntaxKind::ListTypeRangeMax);
        }
    } else if p.at(SyntaxKind::Integer) {
        let exact = p.start();
        p.bump(SyntaxKind::Integer);
        exact.complete(p, SyntaxKind::ListTypeRangeExact);
    } else if p.at(T![..]) {
        // open ended
        p.bump(T![..]);
    }

    m.complete(p, SyntaxKind::ListTypeRange);
}

fn list_type(
    p: &mut Parser,
    types: impl Fn(&mut Parser) -> Option<CompletedMarker>,
) -> CompletedMarker {
    assert!(p.at(T!['[']));

    let m = p.start();
    p.expect(T!['[']);

    if types(p).is_none() {
        p.error(ExpectedError::report(p.pos, Expected::Type));
    }

    if p.eat(T![;]) {
        // nothing is the same as `..`
        list_type_range(p);
    }

    p.expect(T![']']);

    m.complete(p, SyntaxKind::ListType)
}

fn anon_record_type(p: &mut Parser, id_only: bool) -> CompletedMarker {
    assert!(p.at(T!['{']));

    let m = p.start();
    p.bump(T!['{']);

    while !p.eof() && !p.at(T!['}']) {
        let m = p.start();

        attributes::outer_attrs(p);

        let key = if p.at(T!['[']) && !id_only {
            list_type(p, ref_expr);

            true
        } else {
            ref_expr(p).is_some()
        };

        if key {
            if p.eat(T![=]) {
                // TODO: AST needs to const evaluate this ~> or send to the type-crate once ready
                expr(p);
            }

            m.complete(p, SyntaxKind::RecordTypeField);
        } else {
            p.err_and_bump(ExpectedError::report(p.pos, Expected::Reference));
            m.abandon(p);
        }

        if !p.at(T!['}']) {
            p.expect(T![,]);
        }
    }

    p.expect(T!['}']);
    m.complete(p, SyntaxKind::RecordType)
}

/// Property-Types:
///     * List<..>
///     * Reference (Data-Type)
///     * Anonymous Record
///     * Union<..>
pub(super) fn prop_type(p: &mut Parser) -> Option<CompletedMarker> {
    let mut lhs = if p.at(T!['[']) {
        list_type(p, prop_type)
    } else if p.at(T!['{']) {
        anon_record_type(p, false)
    } else if let Some(m) = ref_expr(p) {
        m
    } else {
        return None;
    };

    while p.at(T![|]) && !p.eof() {
        let m = lhs.precede(p);

        p.expect(T![|]);

        if p.at(T!['[']) {
            list_type(p, prop_type);
        } else if p.at(T!['{']) {
            // TODO: this should have no default
            anon_record_type(p, false);
        } else if ref_expr(p).is_some() {
            // ref_expr already makes sure that it is a reference, which means that we do not
            // need to execute code
        } else {
            p.err_and_bump(ExpectedError::report(p.pos, Expected::Type));
        };

        lhs = m.complete(p, SyntaxKind::UnionType);
    }

    Some(lhs)
}

pub(super) fn entity_type(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(T!['{']) {
        // TODO: this must allow links ~> therefore cannot be reused
        Some(anon_record_type(p, false))
    } else {
        None
    }
}

pub(super) fn data_type(p: &mut Parser) {
    // TODO: this should have a required default value
}
