//! Literal types.

use crate::span::Span;

/// A Java literal value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    Int(IntLit),
    Float(FloatLit),
    Bool(BoolLit),
    Char(CharLit),
    Str(StrLit),
    Null(NullLit),
}

impl Lit {
    pub fn span(&self) -> Span {
        match self {
            Lit::Int(l) => l.span,
            Lit::Float(l) => l.span,
            Lit::Bool(l) => l.span,
            Lit::Char(l) => l.span,
            Lit::Str(l) => l.span,
            Lit::Null(l) => l.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntLit {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FloatLit {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolLit {
    pub value: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharLit {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StrLit {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullLit {
    pub span: Span,
}
