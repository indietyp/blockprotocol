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
    grammar::{attributes, expressions::atom, BlockLike},
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
    Neither,
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

fn lhs(p: &mut Parser) -> Option<(CompletedMarker, BlockLike)> {
    match Affix::fetch(p) {
        //> test expr unary
        Affix::Prefix(kind, bp) => {
            let m = p.start();
            p.bump(kind);

            expr_bp(p, None, bp);

            let cm = m.complete(p, SyntaxKind::PrefixExpr);

            Some((cm, BlockLike::NotBlock))
        }
        _ => {
            let m;

            //> test expr full_range
            for op in [T![..=], T![..]] {
                if p.at(op) {
                    m = p.start();
                    p.bump(op);
                    if p.at_ts(LHS_FIRST) {
                        // TODO
                        expr_bp(p, None, Precedence(2));
                    }
                    let cm = m.complete(p, SyntaxKind::RangeExpr);
                    return Some((cm, BlockLike::NotBlock));
                }
            }

            let (lhs, blocklike) = atom::atom_expr(p, r)?;
            let (cm, block_like) =
                postfix_expr(p, lhs, blocklike, !(r.prefer_stmt && blocklike.is_block()));
            return Some((cm, block_like));
        }
    }
}

fn expr_bp(p: &mut Parser, m: Option<Marker>, min_bp: Precedence) {
    let m = m.unwrap_or_else(|| {
        let m = p.start();
        attributes::outer_attrs(p);
        m
    });
}
