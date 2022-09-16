use crate::{
    grammar::{expressions::pratt::expr_bp, Precedence},
    parser::Parser,
};

pub(crate) mod atom;
pub(crate) mod postfix;
pub(crate) mod pratt;

// TODO: change how we handle `{}` and blocks!

// for now this is empty
#[derive(Clone, Copy)]
pub(crate) struct Restrictions {
    naked_record: bool,
}

pub(super) fn expr(p: &mut Parser<'_>) -> bool {
    let r = Restrictions {
        naked_record: false,
    };
    expr_bp(p, None, r, Precedence(1)).is_some()
}
