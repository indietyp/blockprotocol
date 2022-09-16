//! Tree structures, this has been adapted from
//! https://github.com/rust-lang/rust-analyzer/blob/6b163c301f70d0e1246fb898b5f5edcc4d03fa4c/crates/syntax/src/syntax_node.rs

use error_stack::Report;
use num_traits::{FromPrimitive, ToPrimitive};
use parser::kind::SyntaxKind;
use rowan::{GreenNode, GreenNodeBuilder, Language};

use crate::{error::SyntaxError, parse::Parse};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PikaLanguage {}

impl Language for PikaLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        SyntaxKind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<PikaLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<PikaLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<PikaLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<PikaLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<PikaLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<PikaLanguage>;

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<Report<SyntaxError>>,
    inner: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub(crate) fn finish_raw(self) -> (GreenNode, Option<Report<SyntaxError>>) {
        let green = self.inner.finish();
        let errors = self.errors.into_iter().fold(None, |acc, value| match acc {
            None => Some(value),
            Some(other) => {
                other.extend_one(value);

                Some(other)
            }
        });

        (green, errors)
    }

    pub fn finish(self) -> Parse<SyntaxNode> {
        let (green, errors) = self.finish_raw();
        todo!()
        // TODO
        // // Disable block validation, see https://github.com/rust-lang/rust-analyzer/pull/10357
        // if cfg!(debug_assertions) && false {
        //     let node = SyntaxNode::new_root(green.clone());
        //     crate::validation::validate_block_structure(&node);
        // }
        // Parse::new(green, errors)
    }

    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let kind = PikaLanguage::kind_to_raw(kind);
        self.inner.token(kind, text);
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        let kind = PikaLanguage::kind_to_raw(kind);
        self.inner.start_node(kind);
    }

    pub fn finish_node(&mut self) {
        self.inner.finish_node();
    }

    // TODO
    // pub fn error(&mut self, error: String, text_pos: TextSize) {
    //     self.errors
    //         .push(SyntaxError::new_at_offset(error, text_pos));
    // }
}
