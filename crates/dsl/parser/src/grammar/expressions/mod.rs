use crate::{
    grammar::{expressions::pratt::expr_bp, Precedence},
    parser::Parser,
};

pub(crate) mod atom;
pub(crate) mod postfix;
pub(crate) mod pratt;

// for now this is empty
#[derive(Clone, Copy)]
struct Restrictions {}

pub(super) fn expr(p: &mut Parser<'_>) -> bool {
    let r = Restrictions {};
    expr_bp(p, None, r, Precedence(1)).is_some()
}
