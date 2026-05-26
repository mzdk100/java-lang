//! Generic type parameter types.

use crate::{ident::Ident, span::Span};

use super::{attribute::Annotation, path::Path};

/// Type parameters: `<T extends Comparable<T>, U>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParameters {
    pub params: Vec<TypeParameter>,
    pub lt_span: Span,
    pub gt_spans: Vec<Span>,
}

/// A single type parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParameter {
    pub annotations: Vec<Annotation>,
    pub name: Ident,
    pub bound: Option<TypeBound>,
    pub span: Span,
}

/// A type bound: `extends Type & Type2`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeBound {
    pub first: Path,
    pub additional: Vec<Path>,
    pub extends_span: Span,
}
