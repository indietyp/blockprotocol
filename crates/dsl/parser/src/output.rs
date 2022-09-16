//! See [`Output`]
//! Adapted from
//! https://github.com/rust-lang/rust-analyzer/blob/6b163c301f70d0e1246fb898b5f5edcc4d03fa4c/crates/parser/src/output.rs

use error_stack::Report;
use num_traits::FromPrimitive;

use crate::{error::ParserError, kind::SyntaxKind};

/// Output of the parser -- a DFS traversal of a concrete syntax tree.
///
/// Use the [`Output::iter`] method to iterate over traversal steps and consume
/// a syntax tree.
///
/// In a sense, this is just a sequence of [`SyntaxKind`]-colored parenthesis
/// interspersed into the original [`crate::Input`]. The output is fundamentally
/// coordinated with the input and `n_input_tokens` refers to the number of
/// times [`crate::Input::push`] was called.
#[derive(Default)]
pub struct Output {
    /// 32-bit encoding of events. If LSB is zero, then that's an index into the
    /// error vector. Otherwise, it's one of the thee other variants, with data encoded as
    ///
    ///     |16 bit kind|8 bit n_input_tokens|4 bit tag|4 bit leftover|
    event: Vec<u32>,
    error: Vec<Report<ParserError>>,
}

#[derive(Debug)]
pub enum Step<'a> {
    Token {
        kind: SyntaxKind,
        n_input_tokens: u8,
    },
    Enter {
        kind: SyntaxKind,
    },
    Exit,
    Error(&'a Report<ParserError>),
}

impl Output {
    pub fn iter(&self) -> impl Iterator<Item = Step<'_>> {
        self.event.iter().map(|&event| {
            if event & 0b1 == 0 {
                return Step::Error(&self.error[(event as usize) >> 1]);
            }
            let tag = ((event & 0x0000_00F0) >> 4) as u8;
            match tag {
                0 => {
                    let kind = SyntaxKind::from_u16(((event & 0xFFFF_0000) >> 16) as u16).unwrap();
                    let n_input_tokens = ((event & 0x0000_FF00) >> 8) as u8;
                    Step::Token {
                        kind,
                        n_input_tokens,
                    }
                }
                1 => {
                    let kind = SyntaxKind::from_u16(((event & 0xFFFF_0000) >> 16) as u16).unwrap();
                    Step::Enter { kind }
                }
                2 => Step::Exit,
                _ => unreachable!(),
            }
        })
    }

    pub(crate) fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        let e = ((kind as u16 as u32) << 16) | ((n_tokens as u32) << 8) | (0 << 4) | 1;
        self.event.push(e);
    }

    pub(crate) fn enter_node(&mut self, kind: SyntaxKind) {
        let e = (u32::from(kind as u16) << 16) | (1 << 4) | 1;
        self.event.push(e);
    }

    pub(crate) fn leave_node(&mut self) {
        let e = 2 << 4 | 1;
        self.event.push(e);
    }

    pub(crate) fn error(&mut self, report: Report<ParserError>) {
        let idx = self.error.len();
        self.error.push(report);
        let e = (idx as u32) << 1;
        self.event.push(e);
    }
}
