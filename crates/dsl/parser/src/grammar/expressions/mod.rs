use crate::{
    grammar::{expressions::pratt::expr_bp, Precedence},
    parser::Parser,
};

pub(crate) mod atom;
pub(crate) mod postfix;
pub(crate) mod pratt;

// TODO: implement proposal I.3

#[derive(Clone, Copy)]
pub(crate) struct Restrictions {}

pub(super) fn expr(p: &mut Parser) -> bool {
    let r = Restrictions {};
    expr_bp(p, None, r, Precedence(1)).is_some()
}
