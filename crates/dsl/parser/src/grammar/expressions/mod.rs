use crate::{
    grammar::{expressions::pratt::expr_bp, Precedence},
    parser::Parser,
};

pub(crate) mod atom;
pub(crate) mod postfix;
pub(crate) mod pratt;

// for now this is empty
#[derive(Clone, Copy)]
pub(crate) struct Restrictions {}

// record type: S {ref-ref = default}
// record expr: S {ref-ref: 12, ref-ref: 12}
// map type: {[string]: string}
// map expr: {ident: 12}

pub(super) fn expr(p: &mut Parser<'_>) -> bool {
    let r = Restrictions {};
    expr_bp(p, None, r, Precedence(1)).is_some()
}
