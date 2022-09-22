//! Adapted from:
//! https://github.com/rust-lang/rust-analyzer/blob/6b163c301f70d0e1246fb898b5f5edcc4d03fa4c/crates/parser/src/parser.rs

use std::cell::Cell;

use error_stack::{Context, Report};
use limit::Limit;

use crate::{
    error::{Expected, ExpectedError, ParserError},
    event::{Event, Events},
    input::Input,
    kind::SyntaxKind,
    marker::Marker,
    token_set::TokenSet,
};

static PARSER_STEP_LIMIT: Limit = Limit::new(15_000_000);

/// `Parser` struct provides the low-level API for
/// navigating through the stream of tokens and
/// constructing the parse tree. The actual parsing
/// happens in the [`grammar`](super::grammar) module.
///
/// However, the result of this `Parser` is not a real
/// tree, but rather a flat stream of events of the form
/// "start expression, consume number literal,
/// finish expression". See `Event` docs for more.
pub(crate) struct Parser<'t> {
    pub(crate) inp: &'t Input,
    pub(crate) pos: usize,
    pub(crate) events: Vec<Event>,
    steps: Cell<u32>,
}

impl<'t> Parser<'t> {
    pub(super) const fn new(inp: &'t Input) -> Parser<'t> {
        Parser {
            inp,
            pos: 0,
            events: Vec::new(),
            steps: Cell::new(0),
        }
    }

    pub(crate) const fn position(&self) -> usize {
        self.pos
    }

    pub(crate) fn finish(self) -> Events {
        Events::new(self.events)
    }

    /// Returns the kind of the current token.
    /// If parser has already reached the end of input,
    /// the special `EOF` kind is returned.
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// Lookahead operation: returns the kind of the next nth
    /// token.
    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        assert!(n <= 3);

        let steps = self.steps.get();
        assert!(
            PARSER_STEP_LIMIT.check(steps as usize).is_ok(),
            "the parser seems stuck"
        );
        self.steps.set(steps + 1);

        self.inp.kind(self.pos + n)
    }

    /// Checks if the current token is `kind`.
    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    // Consume the next token if `kind` matches.
    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        let n_raw_tokens = kind.n_raw_tokens();
        self.do_bump(kind, n_raw_tokens);
        true
    }

    pub(crate) fn eof(&self) -> bool {
        self.at(SyntaxKind::EndOfFile)
    }

    pub(crate) fn at_composite2(&self, n: usize, k1: SyntaxKind, k2: SyntaxKind) -> bool {
        self.inp.kind(self.pos + n) == k1
            && self.inp.kind(self.pos + n + 1) == k2
            && self.inp.is_joint(self.pos + n)
    }

    pub(crate) fn at_composite3(
        &self,
        n: usize,
        k1: SyntaxKind,
        k2: SyntaxKind,
        k3: SyntaxKind,
    ) -> bool {
        self.inp.kind(self.pos + n) == k1
            && self.inp.kind(self.pos + n + 1) == k2
            && self.inp.kind(self.pos + n + 2) == k3
            && self.inp.is_joint(self.pos + n)
            && self.inp.is_joint(self.pos + n + 1)
    }

    /// Checks if the current token is in `kinds`.
    pub(crate) fn at_ts(&self, kinds: TokenSet) -> bool {
        kinds.contains(self.current())
    }

    /// Checks if the current token is contextual keyword with text `t`.
    pub(crate) fn at_contextual_kw(&self, kw: SyntaxKind) -> bool {
        self.inp.contextual_kind(self.pos) == kw
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    pub(crate) fn start(&mut self) -> Marker {
        #[expect(clippy::cast_possible_truncation)]
        let pos = self.events.len() as u32;
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }

    /// Consume the next token if `kind` matches.
    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    /// Advances the parser by one token
    pub(crate) fn bump_any(&mut self) {
        let kind = self.nth(0);
        if kind == SyntaxKind::EndOfFile {
            return;
        }
        self.do_bump(kind, 1);
    }

    /// Advances the parser by one token, remapping its kind.
    /// This is useful to create contextual keywords from
    /// identifiers. For example, the lexer creates a `union`
    /// *identifier* token, but the parser remaps it to the
    /// `union` keyword, and keyword is what ends up in the
    /// final tree.
    pub(crate) fn bump_remap(&mut self, kind: SyntaxKind) {
        if self.nth(0) == SyntaxKind::EndOfFile {
            // FIXME: panic!?
            return;
        }
        self.do_bump(kind, 1);
    }

    /// Emit error with the `message`
    /// TODO: we need support for spans
    pub(crate) fn error(&mut self, message: Report<impl Context>) {
        self.push_event(Event::Error(message.change_context(ParserError)));
    }

    /// Consume the next token if it is `kind` or emit an error
    /// otherwise.
    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        // TODO: span
        // TODO: expect collect
        self.error(ExpectedError::report(self.pos, Expected::Kind(kind)));
        false
    }

    /// Create an error node and consume the next token.
    pub(crate) fn err_and_bump(&mut self, message: Report<impl Context>) {
        self.err_recover(message.change_context(ParserError), TokenSet::EMPTY);
    }

    /// Create an error node and consume the next token.
    /// We only every recover until we spot a recovery token, these are specified in `recovery`
    pub(crate) fn err_recover(&mut self, message: Report<impl Context>, recovery: TokenSet) {
        match self.current() {
            T!['{'] | T!['}'] => {
                self.error(message);
                return;
            }
            _ => (),
        }

        if self.at_ts(recovery) {
            self.error(message);
            return;
        }

        let m = self.start();
        self.error(message);
        self.bump_any();
        m.complete(self, SyntaxKind::Error);
    }

    fn do_bump(&mut self, kind: SyntaxKind, n_raw_tokens: u8) {
        self.pos += n_raw_tokens as usize;
        self.push_event(Event::Token { kind, n_raw_tokens });
    }

    pub(crate) fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }
}
