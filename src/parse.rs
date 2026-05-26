use std::cell::{Cell, RefCell};

use crate::{
    ast::{Comment, CommentKind},
    error::{Error, Result},
    ident::Ident,
    lexer,
    span::Span,
    token::{Token, TokenKind},
};

/// Saved parser state for speculative parsing with backtracking.
pub struct ParserState {
    cursor: usize,
    pending_gts: u8,
}

/// The parsing cursor, similar to syn's `ParseBuffer`.
///
/// This is the primary interface for consuming tokens during parsing.
pub struct ParseStream<'a> {
    tokens: &'a [Token],
    cursor: Cell<usize>,
    errors: RefCell<Vec<Error>>,
    /// Pending greater-than tokens from splitting >> or >>> for nested generics
    pending_gts: Cell<u8>,
    /// Pre-allocated synthetic `>` token for >> / >>> splitting
    synthetic_gt: Token,
    /// Comments skipped by skip_comments(), available for collection
    pending_comments: RefCell<Vec<Comment>>,
}

impl<'a> ParseStream<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        ParseStream {
            tokens,
            cursor: Cell::new(0),
            errors: RefCell::new(Vec::new()),
            pending_gts: Cell::new(0),
            synthetic_gt: Token {
                kind: TokenKind::Gt,
                span: Span::new(0, 0),
            },
            pending_comments: RefCell::new(Vec::new()),
        }
    }

    /// Returns true if there are no more tokens to parse (except EOF).
    pub fn is_empty(&self) -> bool {
        self.skip_comments();
        let cursor = self.cursor.get();
        cursor >= self.tokens.len() - 1
    }

    /// Returns true if there are no more tokens (including comments) except EOF.
    pub fn is_empty_raw(&self) -> bool {
        let cursor = self.cursor.get();
        cursor >= self.tokens.len() - 1
    }

    /// Peek at the current token without consuming it.
    /// Comment tokens are skipped transparently.
    pub fn peek(&self) -> &Token {
        if self.pending_gts.get() > 0 {
            return &self.synthetic_gt;
        }
        self.skip_comments();
        let cursor = self.cursor.get();
        &self.tokens[cursor.min(self.tokens.len() - 1)]
    }

    /// Peek at the raw token at the cursor without skipping comments.
    pub fn peek_raw(&self) -> &Token {
        let cursor = self.cursor.get();
        &self.tokens[cursor.min(self.tokens.len() - 1)]
    }

    /// Get the current cursor position.
    pub fn cursor(&self) -> usize {
        self.cursor.get()
    }

    /// Set the current cursor position (used for backtracking).
    pub fn set_cursor(&self, pos: usize) {
        self.cursor.set(pos);
    }

    /// Save the full parser state (cursor + pending_gts) for speculative parsing.
    pub fn save_state(&self) -> ParserState {
        ParserState {
            cursor: self.cursor.get(),
            pending_gts: self.pending_gts.get(),
        }
    }

    /// Restore the full parser state (cursor + pending_gts) after speculative parsing.
    pub fn restore_state(&self, state: ParserState) {
        self.cursor.set(state.cursor);
        self.pending_gts.set(state.pending_gts);
    }

    /// Advance past the current token and return it.
    /// Comment tokens are skipped transparently.
    fn advance(&self) -> &Token {
        if self.pending_gts.get() > 0 {
            self.pending_gts.set(self.pending_gts.get() - 1);
            return &self.synthetic_gt;
        }
        let cursor = self.cursor.get();
        let tok = &self.tokens[cursor];
        if cursor < self.tokens.len() - 1 {
            self.cursor.set(cursor + 1);
        }
        self.skip_comments();
        tok
    }

    /// Push an error to the error list.
    pub fn error<T: std::fmt::Display>(&self, span: Span, msg: T) -> Error {
        let err = Error::new(span, msg);
        self.errors.borrow_mut().push(err.clone());
        err
    }

    /// Split a >> token into two > tokens for nested generic parsing.
    pub fn split_gt(&self) {
        let cursor = self.cursor.get();
        let kind = &self.tokens[cursor.min(self.tokens.len() - 1)].kind;
        match kind {
            TokenKind::GtGt => {
                self.advance();
                self.pending_gts.set(self.pending_gts.get() + 1);
            }
            TokenKind::GtGtGt => {
                self.advance();
                self.pending_gts.set(self.pending_gts.get() + 2);
            }
            _ => {}
        }
    }

    /// Consume and return the current token regardless of its kind.
    pub fn next(&self) -> &Token {
        self.advance()
    }

    /// Check if the current token matches the given kind.
    pub fn is(&self, kind: &TokenKind) -> bool {
        &self.peek().kind == kind
    }

    /// Check if the current token is an identifier with the given name.
    pub fn is_ident(&self, name: &str) -> bool {
        match &self.peek().kind {
            TokenKind::Ident(s) => s == name,
            _ => false,
        }
    }

    /// Check if the current token is any identifier (including contextual keywords).
    pub fn is_any_ident(&self) -> bool {
        matches!(
            &self.peek().kind,
            TokenKind::Ident(_)
                | TokenKind::Record
                | TokenKind::Sealed
                | TokenKind::Var
                | TokenKind::Yield
                | TokenKind::Open
                | TokenKind::Provides
                | TokenKind::Requires
                | TokenKind::Uses
                | TokenKind::With
                | TokenKind::When
                | TokenKind::To
                | TokenKind::Exports
                | TokenKind::Opens
                | TokenKind::Transitive
                | TokenKind::Permits
                | TokenKind::NonSealed
                | TokenKind::Module
                | TokenKind::Byte
                | TokenKind::Short
                | TokenKind::Int
                | TokenKind::Long
                | TokenKind::Char
                | TokenKind::Float
                | TokenKind::Double
                | TokenKind::Boolean
                | TokenKind::Void
        )
    }

    /// Check if the current token can be used as a type name.
    pub fn is_type_ident(&self) -> bool {
        self.is_any_ident() || self.is(&TokenKind::At)
    }

    /// Check if the current token is a given keyword.
    pub fn is_keyword(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    /// Consume the next token if it matches the expected kind.
    pub fn eat(&self, expected: &TokenKind) -> bool {
        if self.is(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// If the current token matches the given kind, consume it.
    /// Otherwise, report an error.
    pub fn expect(&self, kind: TokenKind) -> Result<()> {
        if self.is(&kind) {
            self.advance();
            Ok(())
        } else {
            Err(Error::expected_token(self.peek().span, &kind.to_string()))
        }
    }

    /// Look ahead `n` tokens (skipping comments).
    pub fn look_ahead(&self, n: usize) -> &Token {
        let mut pos = self.cursor.get();
        let mut remaining = n;
        while pos < self.tokens.len() - 1 {
            if !is_comment_token(&self.tokens[pos].kind) {
                if remaining == 0 {
                    break;
                }
                remaining -= 1;
            }
            pos += 1;
        }
        &self.tokens[pos.min(self.tokens.len() - 1)]
    }

    /// Parse a comma-separated list of items terminated by some token.
    pub fn parse_terminated<T, F>(&self, mut parse_item: F) -> Result<Vec<T>>
    where
        F: FnMut(&ParseStream) -> Result<T>,
    {
        let mut items = Vec::new();
        if self.is_empty() || !can_start_item(&self.peek().kind) {
            return Ok(items);
        }
        loop {
            items.push(parse_item(self)?);
            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }
        Ok(items)
    }

    /// Parse zero or more items separated by commas, not requiring a terminator.
    pub fn parse_separated<T, F>(
        &self,
        can_start_fn: fn(&TokenKind) -> bool,
        mut parse_item: F,
    ) -> Result<Vec<T>>
    where
        F: FnMut(&ParseStream) -> Result<T>,
    {
        let mut items = Vec::new();
        if self.is_empty() || !can_start_fn(&self.peek().kind) {
            return Ok(items);
        }
        loop {
            items.push(parse_item(self)?);
            if !self.eat(&TokenKind::Comma) {
                break;
            }
            if self.is_empty() {
                break;
            }
        }
        Ok(items)
    }

    /// Try to parse something. If parsing fails, revert the cursor.
    pub fn try_parse<T, F>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&ParseStream) -> Result<T>,
    {
        let saved = self.cursor.get();
        match f(self) {
            Ok(t) => Some(t),
            Err(_) => {
                self.cursor.set(saved);
                None
            }
        }
    }

    /// Parse something inside parentheses.
    pub fn parse_parenthesized<T, F>(&self, mut f: F) -> Result<T>
    where
        F: FnMut(&ParseStream) -> Result<T>,
    {
        self.expect(TokenKind::LParen)?;
        let result = f(self)?;
        self.expect(TokenKind::RParen)?;
        Ok(result)
    }

    /// Parse any type that implements `Parse`.
    pub fn parse<T: Parse>(&self) -> Result<T> {
        T::parse(self)
    }

    /// Parse something inside braces.
    pub fn parse_braced<T, F>(&self, mut f: F) -> Result<T>
    where
        F: FnMut(&ParseStream) -> Result<T>,
    {
        self.expect(TokenKind::LBrace)?;
        let result = f(self)?;
        self.expect(TokenKind::RBrace)?;
        Ok(result)
    }

    /// Parse something inside brackets.
    pub fn parse_bracketed<T, F>(&self, mut f: F) -> Result<T>
    where
        F: FnMut(&ParseStream) -> Result<T>,
    {
        self.expect(TokenKind::LBracket)?;
        let result = f(self)?;
        self.expect(TokenKind::RBracket)?;
        Ok(result)
    }

    /// Consume the expected token, then return the span of the raw token at the cursor.
    /// Unlike `expect(kind); peek().span`, this does NOT skip comments after consuming.
    pub fn expect_then_raw_span(&self, kind: TokenKind) -> Result<Span> {
        self.expect(kind)?;
        Ok(self.peek_raw().span)
    }
    /// Also accepts contextual keywords (record, sealed, var, yield, open, etc.)
    pub fn parse_ident(&self) -> Result<Ident> {
        match &self.peek().kind {
            TokenKind::Ident(s) => {
                let span = self.peek().span;
                self.advance();
                Ok(Ident::new(s.clone(), span))
            }
            TokenKind::Record
            | TokenKind::Sealed
            | TokenKind::Var
            | TokenKind::Yield
            | TokenKind::Open
            | TokenKind::Provides
            | TokenKind::Requires
            | TokenKind::Uses
            | TokenKind::With
            | TokenKind::When
            | TokenKind::To
            | TokenKind::Exports
            | TokenKind::Opens
            | TokenKind::Transitive
            | TokenKind::Permits
            | TokenKind::NonSealed
            | TokenKind::Module
            | TokenKind::Byte
            | TokenKind::Short
            | TokenKind::Int
            | TokenKind::Long
            | TokenKind::Char
            | TokenKind::Float
            | TokenKind::Double
            | TokenKind::Boolean
            | TokenKind::Void => {
                let name = format!("{}", self.peek().kind);
                let span = self.peek().span;
                self.advance();
                Ok(Ident::new(name, span))
            }
            _other => Err(Error::expected_token(self.peek().span, "identifier")),
        }
    }

    /// Take any errors accumulated during parsing.
    pub fn take_errors(&self) -> Vec<Error> {
        self.errors.borrow_mut().drain(..).collect()
    }

    /// Create a span covering from the given start to the current position.
    pub fn span_since(&self, start: Span) -> Span {
        let end = if self.cursor.get() > 0 {
            self.tokens[self.cursor.get() - 1].span
        } else {
            start
        };
        start.join(end)
    }

    /// Skip past any comment tokens at the current cursor position.
    /// This is the public version for explicit comment skipping.
    pub fn skip_comments_to_peek(&self) {
        self.skip_comments();
    }

    fn skip_comments(&self) {
        while self.cursor.get() < self.tokens.len()
            && is_comment_token(&self.tokens[self.cursor.get()].kind)
        {
            let tok = &self.tokens[self.cursor.get()];
            self.pending_comments
                .borrow_mut()
                .push(token_to_comment(tok));
            self.cursor.set(self.cursor.get() + 1);
        }
    }

    /// Collect pending doc comments (skipped by peek/advance).
    /// Returns only doc comments (/// and /** */), discards regular comments.
    pub fn collect_pending_doc_comments(&self) -> Vec<Comment> {
        let all = self
            .pending_comments
            .borrow_mut()
            .drain(..)
            .collect::<Vec<_>>();
        all.into_iter()
            .filter(|c| c.kind == CommentKind::DocLine || c.kind == CommentKind::DocBlock)
            .collect()
    }

    /// Collect all pending comments (skipped by peek/advance).
    pub fn collect_pending_comments(&self) -> Vec<Comment> {
        self.pending_comments.borrow_mut().drain(..).collect()
    }

    /// Collect and consume leading doc comments (/// and /** */).
    /// Regular comments (// and /* */) are skipped.
    pub fn collect_leading_doc_comments(&self) -> Vec<Comment> {
        let mut comments = Vec::new();
        while self.cursor.get() < self.tokens.len() {
            match &self.tokens[self.cursor.get()].kind {
                TokenKind::DocLineComment(_) | TokenKind::DocBlockComment(_) => {
                    let tok = &self.tokens[self.cursor.get()];
                    comments.push(token_to_comment(tok));
                    self.cursor.set(self.cursor.get() + 1);
                }
                TokenKind::LineComment(_) | TokenKind::BlockComment(_) => {
                    // Skip regular comments
                    self.cursor.set(self.cursor.get() + 1);
                }
                _ => break,
            }
        }
        comments
    }

    /// Collect and consume all leading comments (both doc and regular).
    pub fn collect_leading_comments(&self) -> Vec<Comment> {
        let mut comments = Vec::new();
        while self.cursor.get() < self.tokens.len() {
            match &self.tokens[self.cursor.get()].kind {
                TokenKind::LineComment(_)
                | TokenKind::BlockComment(_)
                | TokenKind::DocLineComment(_)
                | TokenKind::DocBlockComment(_) => {
                    let tok = &self.tokens[self.cursor.get()];
                    comments.push(token_to_comment(tok));
                    self.cursor.set(self.cursor.get() + 1);
                }
                _ => break,
            }
        }
        comments
    }
}

fn is_comment_token(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::LineComment(_)
            | TokenKind::BlockComment(_)
            | TokenKind::DocLineComment(_)
            | TokenKind::DocBlockComment(_)
    )
}

fn token_to_comment(tok: &Token) -> Comment {
    let kind = match &tok.kind {
        TokenKind::DocLineComment(_) => CommentKind::DocLine,
        TokenKind::DocBlockComment(_) => CommentKind::DocBlock,
        TokenKind::LineComment(_) => CommentKind::Line,
        TokenKind::BlockComment(_) => CommentKind::Block,
        _ => unreachable!(),
    };
    Comment {
        kind,
        span: tok.span,
    }
}

fn can_start_item(kind: &TokenKind) -> bool {
    !matches!(
        kind,
        TokenKind::Eof
            | TokenKind::RParen
            | TokenKind::RBrace
            | TokenKind::RBracket
            | TokenKind::Semicolon
            | TokenKind::Comma
    )
}

/// A trait for types that can be parsed from a `ParseStream`.
///
/// This is the core parsing trait, analogous to syn's `Parse` trait.
///
/// # Example
///
/// ```
/// use java_lang::{Parse, ParseStream, parse_str, Ident};
///
/// struct SimpleName {
///     name: Ident,
/// }
///
/// impl Parse for SimpleName {
///     fn parse(input: &ParseStream) -> java_lang::Result<Self> {
///         Ok(SimpleName {
///             name: input.parse_ident()?,
///         })
///     }
/// }
/// ```
pub trait Parse: Sized {
    /// Parse this type from the given `ParseStream`.
    fn parse(input: &ParseStream) -> Result<Self>;
}

impl Parse for Ident {
    fn parse(input: &ParseStream) -> Result<Self> {
        input.parse_ident()
    }
}

/// Parse a string into a value of type `T` that implements `Parse`.
pub fn parse_str<T: Parse>(s: &str) -> Result<T> {
    let tokens = lexer::tokenize(s);
    let stream = ParseStream::new(&tokens);
    let result = T::parse(&stream)?;
    // Check for trailing tokens
    if !stream.is_empty() {
        return Err(Error::new(stream.peek().span, "unexpected trailing tokens"));
    }
    Ok(result)
}

/// Parse a string into a value of type `T` without checking for trailing tokens.
pub fn parse<T: Parse>(s: &str) -> Result<T> {
    let tokens = lexer::tokenize(s);
    let stream = ParseStream::new(&tokens);
    T::parse(&stream)
}

/// Parse a Java source file into a value of type `T`.
pub fn parse_file<T: Parse>(path: &std::path::Path) -> Result<T> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| Error::new(Span::call_site(), format!("failed to read file: {}", e)))?;
    parse_str(&content)
}

/// Peek at the next token and check if it matches.
#[macro_export]
macro_rules! peek {
    ($stream:expr, $kind:ident) => {
        $stream.is(&$crate::token::TokenKind::$kind)
    };
}

/// Peek at the next token and check if it is an identifier.
#[macro_export]
macro_rules! peek_ident {
    ($stream:expr) => {
        $stream.is_any_ident()
    };
}

/// Optionally parse something. Returns `None` if the next token doesn't match.
#[macro_export]
macro_rules! opt {
    ($stream:expr, $method:ident $(, $arg:expr)*) => {
        if $stream.is_empty() {
            None
        } else {
            match $stream.$method($($arg),*) {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        }
    };
}
