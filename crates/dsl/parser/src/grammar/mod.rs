use crate::{parser::Parser, SyntaxKind};

#[derive(PartialEq, Eq)]
pub(super) enum Semicolon {
    Required,
    Optional,
    Forbidden,
}

pub(crate) mod entry {
    use super::*;

    pub(crate) mod prefix {
        use super::*;

        pub(crate) fn block(p: &mut Parser<'_>) {
            expressions::block_expr(p);
        }

        pub(crate) fn stmt(p: &mut Parser<'_>) {
            expressions::stmt(p, Semicolon::Forbidden);
        }

        pub(crate) fn ty(p: &mut Parser<'_>) {
            types::type_(p);
        }
        pub(crate) fn expr(p: &mut Parser<'_>) {
            let _ = expressions::expr(p);
        }
        pub(crate) fn path(p: &mut Parser<'_>) {
            let _ = paths::type_path(p);
        }
        pub(crate) fn item(p: &mut Parser<'_>) {
            items::item_or_macro(p, true);
        }
        // Parse a meta item , which excluded [], e.g : #[ MetaItem ]
        pub(crate) fn attribute(p: &mut Parser<'_>) {
            attributes::meta(p);
        }
    }
}

pub(crate) fn reparser(
    node: SyntaxKind,
    first_child: Option<SyntaxKind>,
    parent: Option<SyntaxKind>,
) -> Option<fn(&mut Parser<'_>)> {
    // let res = match node {
    //     BLOCK_EXPR => expressions::block_expr,
    //     RECORD_FIELD_LIST => items::record_field_list,
    //     RECORD_EXPR_FIELD_LIST => items::record_expr_field_list,
    //     VARIANT_LIST => items::variant_list,
    //     MATCH_ARM_LIST => items::match_arm_list,
    //     USE_TREE_LIST => items::use_tree_list,
    //     EXTERN_ITEM_LIST => items::extern_item_list,
    //     TOKEN_TREE if first_child? == T!['{'] => items::token_tree,
    //     ASSOC_ITEM_LIST => match parent? {
    //         IMPL | TRAIT => items::assoc_item_list,
    //         _ => return None,
    //     },
    //     ITEM_LIST => items::item_list,
    //     _ => return None,
    // };
    // Some(res)
    None
}
