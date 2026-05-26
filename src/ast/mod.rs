pub mod attribute;
pub mod compilation_unit;
pub mod expr;
pub mod generics;
pub mod item;
pub mod lit;
pub mod op;
pub mod pat;
pub mod path;
pub mod stmt;
pub mod ty;

use crate::span::Span;

/// A comment in the source code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    pub kind: CommentKind,
    pub span: Span,
}

/// The kind of comment.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommentKind {
    /// A line comment: `// ...`
    Line,
    /// A block comment: `/* ... */`
    Block,
    /// A documentation line comment: `/// ...`
    DocLine,
    /// A documentation block comment: `/** ... */`
    DocBlock,
}

impl Comment {
    /// Get the source text of this comment from the full source.
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.range()]
    }
}

// Re-export all types for convenience
pub use attribute::*;
pub use compilation_unit::*;
pub use expr::*;
pub use generics::*;
pub use item::*;
pub use lit::*;
pub use op::*;
pub use pat::*;
pub use path::*;
pub use stmt::*;
pub use ty::*;
