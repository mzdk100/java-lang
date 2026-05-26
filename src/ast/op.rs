//! Operator types.

use crate::span::Span;

/// A binary operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    LShift,
    RShift,
    URShift,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    LAnd,
    LOr,
}

/// A binary operator with its span.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinOpToken {
    pub op: BinOp,
    pub span: Span,
}

/// An assignment operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    RemAssign,
    LShiftAssign,
    RShiftAssign,
    URShiftAssign,
}

/// An assignment operator with its span.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignOpToken {
    pub op: AssignOp,
    pub span: Span,
}

/// A unary operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}
