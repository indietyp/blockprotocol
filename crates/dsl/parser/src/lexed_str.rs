//! Adapted from:
//! https://github.com/rust-lang/rust-analyzer/blob/6b163c301f70d0e1246fb898b5f5edcc4d03fa4c/crates/parser/src/lexed_str.rs

use std::ops;

use lexer::{Kind, Lexer, Token};
use text_size::{TextRange, TextSize};

use crate::{input::Input, kind::SyntaxKind};

pub struct LexedStr<'a> {
    text: &'a str,
    tokens: Vec<Token<'a, SyntaxKind>>,
}

impl<'a> LexedStr<'a> {
    pub fn new(text: &'a str) -> LexedStr<'a> {
        let mut conv = Converter::new(text);

        for token in Lexer::new(text) {
            conv.extend_token(token);
        }

        conv.finalize_with_eof()
    }

    pub fn single_token(text: &'a str) -> Option<Token<SyntaxKind>> {
        if text.is_empty() {
            return None;
        }

        let token = Lexer::new(text).next()?;
        if token.text().len() != text.len() {
            return None;
        }

        let mut conv = Converter::new(text);
        conv.extend_token(token);
        match &*conv.res.tokens {
            [kind] => Some(*kind),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        self.text
    }

    pub fn len(&self) -> usize {
        self.tokens.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn token(&self, idx: usize) -> Token<SyntaxKind> {
        assert!(idx < self.len());
        self.tokens[idx]
    }

    pub fn range_text(&self, r: ops::Range<usize>) -> &str {
        assert!(r.start < r.end && r.end <= self.len());
        let lo = u32::from(self.tokens[r.start].range().start()) as usize;
        let hi = u32::from(self.tokens[r.end].range().end()) as usize;
        &self.text[lo..hi]
    }

    fn push(&mut self, token: Token<'a, SyntaxKind>) {
        self.tokens.push(token);
    }
}

struct Converter<'a> {
    res: LexedStr<'a>,
    offset: usize,
}

impl<'a> Converter<'a> {
    const fn new(text: &'a str) -> Self {
        Self {
            res: LexedStr {
                text,
                tokens: vec![],
            },
            offset: 0,
        }
    }

    fn finalize_with_eof(mut self) -> LexedStr<'a> {
        self.res.push(Token::new(
            SyntaxKind::EndOfFile,
            "",
            #[expect(
                clippy::cast_possible_truncation,
                reason = "lexer cannot parse more than u32"
            )]
            TextRange::new(
                TextSize::from(self.offset as u32),
                TextSize::from(self.offset as u32),
            ),
        ));
        self.res
    }

    fn push(&mut self, token: Token<'a, SyntaxKind>) {
        self.res.push(token);
        self.offset += token.text().len();
    }

    fn extend_token(&mut self, token: Token<'a, Kind>) {
        self.push(Token::new(
            SyntaxKind::from(token.kind()),
            token.text(),
            token.range(),
        ));
    }
}

impl<'a> LexedStr<'a> {
    pub fn to_input(&self) -> Input {
        let mut res = Input::default();
        let mut was_joint = false;

        for idx in 0..self.len() {
            let token = self.token(idx);
            let kind = token.kind();

            if kind.is_trivia() {
                was_joint = false;
            } else {
                if kind == SyntaxKind::Ident {
                    let token_text = token.text();
                    let contextual_kw = SyntaxKind::from_contextual_keyword(token_text)
                        .unwrap_or(SyntaxKind::Ident);
                    res.push_ident(contextual_kw);
                } else {
                    if was_joint {
                        res.was_joint();
                    }

                    res.push(kind);
                }

                was_joint = true;
            }
        }

        res
    }
}
