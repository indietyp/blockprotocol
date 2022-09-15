//! This has been adapted from rust-analyser
//! https://github.com/rust-lang/rust-analyzer/blob/6b163c301f70d0e1246fb898b5f5edcc4d03fa4c/crates/syntax/src/lib.rs

use std::{marker::PhantomData, sync::Arc};

use error_stack::Report;
use rowan::GreenNode;

use crate::{error::SyntaxError, tree::SyntaxNode};

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug)]
pub struct Parse<T> {
    green: GreenNode,
    errors: Arc<Vec<Report<SyntaxError>>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Self {
        Parse {
            green: self.green.clone(),
            errors: self.errors.clone(),
            _ty: PhantomData::default(),
        }
    }
}

impl<T> Parse<T> {
    fn new(green: GreenNode, errors: Vec<Report<SyntaxError>>) -> Self {
        Parse {
            green,
            errors: Arc::new(errors),
            _ty: PhantomData::default(),
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn errors(&self) -> &[Report<SyntaxError>] {
        &*self.errors
    }
}

// TODO: AstNode
// TODO: SyntaxNode cast
// TODO: Parse<SourceFile>
