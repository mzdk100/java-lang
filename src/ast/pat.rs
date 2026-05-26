//! Pattern types.

use crate::{ident::Ident, span::Span};

use super::{attribute::Annotation, expr::Expr, ty::Type};

/// A Java pattern (used in instanceof, switch, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    TypePattern(TypePattern),
    RecordPattern(RecordPattern),
}

impl Pattern {
    pub fn span(&self) -> Span {
        match self {
            Pattern::TypePattern(p) => p.span,
            Pattern::RecordPattern(p) => p.span,
        }
    }
}

/// A type pattern: `Type name` or just `Type`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypePattern {
    pub annotations: Vec<Annotation>,
    pub ty: Type,
    pub name: Option<Ident>,
    pub span: Span,
}

/// A record pattern: `Type(comp1, comp2, ...)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordPattern {
    pub record_type: Type,
    pub components: Vec<Pattern>,
    pub span: Span,
}

/// A guard in a switch case pattern: `when expr`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guard {
    pub when_span: Span,
    pub expr: Expr,
}
