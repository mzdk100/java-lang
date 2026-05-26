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
            Self::Int(l) => l.span,
            Self::Float(l) => l.span,
            Self::Bool(l) => l.span,
            Self::Char(l) => l.span,
            Self::Str(l) => l.span,
            Self::Null(l) => l.span,
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
