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

fn union_type(
    p: &mut Parser,
    types: impl Fn(&mut Parser) -> Option<CompletedMarker>,
) -> CompletedMarker {
    let m = p.start();

    loop {
        types(p);

        if !p.eat(T![|]) {
            break;
        }
    }

    m.complete(p, SyntaxKind::UnionType)
}

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

    types(p);

    if p.eat(T![;]) {
        // nothing is the same as `..`
        list_type_range(p);
    }

    p.expect(T![']']);

    m.complete(p, SyntaxKind::ListType)
}

fn anon_record_type(p: &mut Parser, id_only: bool) {
    assert!(p.at(T!['{']));

    let m = p.start();
    p.bump(T!['{']);

    while !p.eof() && !p.at(T!['}']) {
        let m = p.start();

        attributes::outer_attrs(p);

        // TODO: id_only
        if ref_expr(p).is_some() {
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

    // TODO: can also be list of references!
    // TODO: id_only
}
