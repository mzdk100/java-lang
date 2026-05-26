use std::fmt;

use crate::span::Span;

/// The error type for Java parsing.
#[derive(Debug, Clone)]
pub struct Error {
    message: String,
    span: Span,
}

impl Error {
    /// Create a new error with a message and span.
    pub fn new<T: fmt::Display>(span: Span, message: T) -> Self {
        Error {
            message: message.to_string(),
            span,
        }
    }

    /// Create an "expected token" error.
    pub(crate) fn expected_token(span: Span, expected: &str) -> Self {
        Error {
            message: format!("expected {}", expected),
            span,
        }
    }

    /// Get the span where the error occurred.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let loc = self.span.start();
        write!(f, "error at byte offset {}: {}", loc, self.message)
    }
}

impl std::error::Error for Error {}

/// A specialized `Result` type for Java parsing.
pub type Result<T> = std::result::Result<T, Error>;
