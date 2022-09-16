//! Implementation of the algorithm described in:
//! https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
//!
//! This has been adapted from
//! https://github.com/segeljakt/pratt/blob/56e194b52d5defd00ef32f6ada9aad98e0346f95/src/lib.rs
//!
//! Our precedence values are multiplied by 10 to make sure we're able to modify and add new ones
//! easily.
//!
//! TODO: implement the other one D:

use crate::{
    grammar::{
        attributes,
        expressions::{atom, postfix::postfix_expr, Restrictions},
        BlockLike,
    },
    marker::{CompletedMarker, Marker},
    parser::Parser,
    token_set::TokenSet,
    SyntaxKind,
};

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub(crate) struct Precedence(pub u32);

impl Precedence {
    const fn raise(mut self) -> Precedence {
        self.0 += 1;
        self
    }

    const fn lower(mut self) -> Precedence {
        self.0 -= 1;
        self
    }

    const fn normalize(mut self) -> Precedence {
        self.0 *= 10;
        self
    }

    const fn min() -> Precedence {
        Precedence(u32::MIN)
    }

    const fn max() -> Precedence {
        Precedence(u32::MAX)
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Associativity {
    Left,
    Right,
}

pub(crate) struct Affix;

//         <lbp>  <rbp>  <nbp> <kind>
// Nilfix:  MIN |  MIN |  MAX | nud
// Prefix:  MIN |   bp |  MAX | nud
// Postfix:  bp |  MIN |  MAX | led
// InfixL:   bp |   bp | bp+1 | led
// InfixR:   bp | bp-1 | bp+1 | led
// InfixN:   bp |   bp |   bp | led

const LHS_FIRST: TokenSet = atom::ATOM_EXPR_FIRST.union(TokenSet::new(&[T![!], T![.], T![-]]));

fn lhs(p: &mut Parser, r: Restrictions) -> Option<(CompletedMarker, BlockLike)> {
    match Affix::prefix(p) {
        //> test expr unary
        Some((kind, precedence)) => {
            let m = p.start();
            p.bump(kind);

            expr_bp(p, None, r, precedence);

            let cm = m.complete(p, SyntaxKind::PrefixExpr);

            Some((cm, BlockLike::NotBlock))
        }
        _ => {
            let m;

            //> test expr full_range
            for op in [T![..=], T![..]] {
                if p.at(op) {
                    // SAFETY: this in infallible, because it's always defined
                    let precedence = Affix::infix(p).unwrap().2;

                    m = p.start();
                    p.bump(op);

                    if p.at_ts(LHS_FIRST) {
                        expr_bp(p, None, r, precedence.lower());
                    }

                    let cm = m.complete(p, SyntaxKind::RangeExpr);
                    return Some((cm, BlockLike::NotBlock));
                }
            }

            let (lhs, block_like) = atom::atom_expr(p, r)?;
            let (cm, block_like) = postfix_expr(p, lhs, block_like, !block_like.is_block());
            return Some((cm, block_like));
        }
    }
}

pub(super) fn expr_bp(
    p: &mut Parser,
    m: Option<Marker>,
    r: Restrictions,
    precedence: Precedence,
) -> Option<(CompletedMarker, BlockLike)> {
    let m = m.unwrap_or_else(|| {
        let m = p.start();
        attributes::outer_attrs(p);
        m
    });

    let mut lhs = match lhs(p, r) {
        Some((lhs, block_like)) => {
            let lhs = lhs.extend_to(p, m);

            if block_like.is_block() {
                return Some((lhs, BlockLike::Block));
            }

            lhs
        }
        None => {
            m.abandon(p);
            return None;
        }
    };

    loop {
        let bp = Affix::infix(p).map(|(kind, assoc, precedence)| match assoc {
            Associativity::Left => (kind, precedence, precedence.raise()),
            Associativity::Right => (kind, precedence.raise(), precedence),
        });

        if let Some((op, l_bp, r_bp)) = bp {
            let is_range = p.at(T![..]) || p.at(T![..=]);

            if l_bp < precedence {
                break;
            }

            let m = lhs.precede(p);
            p.bump(op);

            if is_range {
                //> test expr postfix_range
                let has_trailing_expression = p.at_ts(LHS_FIRST) && !p.at(T!['{']);
                if !has_trailing_expression {
                    // no RHS
                    lhs = m.complete(p, SyntaxKind::RangeExpr);
                    break;
                }
            }

            expr_bp(p, None, r, r_bp);
            lhs = m.complete(
                p,
                if is_range {
                    SyntaxKind::RangeExpr
                } else {
                    SyntaxKind::InfixExpr
                },
            );
        } else {
            break;
        }
    }

    Some((lhs, BlockLike::NotBlock))
}
