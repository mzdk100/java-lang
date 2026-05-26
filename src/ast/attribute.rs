//! Annotation types.

use crate::span::Span;

use super::expr::Expr;
use super::path::Path;

/// A Java annotation (e.g., `@Override`, `@SuppressWarnings("unchecked")`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Annotation {
    /// `@TypeName`
    Marker { at_token: Span, name: Path },
    /// `@TypeName(value)`
    SingleElement {
        at_token: Span,
        name: Path,
        eq_token: Span,
        value: Box<ElementValue>,
    },
    /// `@TypeName(key1 = val1, key2 = val2)`
    Normal {
        at_token: Span,
        name: Path,
        paren_span: (Span, Span), // (open, close)
        pairs: Vec<ElementValuePair>,
    },
}

impl Annotation {
    pub fn span(&self) -> Span {
        match self {
            Self::Marker { at_token, name } => at_token.join(name.span),
            Self::SingleElement {
                at_token, value, ..
            } => at_token.join(value.span()),
            Self::Normal {
                at_token,
                paren_span,
                ..
            } => at_token.join(paren_span.1),
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            Self::Marker { name, .. } => name,
            Self::SingleElement { name, .. } => name,
            Self::Normal { name, .. } => name,
        }
    }
}

/// A key-value pair in a normal annotation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementValuePair {
    pub key: crate::ident::Ident,
    pub eq_span: Span,
    pub value: ElementValue,
}

/// An element value in an annotation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementValue {
    /// An expression (constant).
    Expr(Expr),
    /// An array initializer.
    Array {
        brace_span: (Span, Span),
        values: Vec<ElementValue>,
        trailing_comma: bool,
    },
    /// A nested annotation.
    Annotation(Annotation),
}

impl ElementValue {
    pub fn span(&self) -> Span {
        match self {
            Self::Expr(e) => e.span(),
            Self::Array { brace_span, .. } => brace_span.0.join(brace_span.1),
            Self::Annotation(a) => a.span(),
        }
    }
}
