//! Type types.

use crate::{ident::Ident, span::Span};

use super::{
    attribute::Annotation,
    path::{Path, TypeArguments},
};

/// A Java type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// A primitive type: `int`, `boolean`, etc.
    Primitive(PrimitiveType),
    /// A reference type: class, interface, type variable, or array.
    Reference(ReferenceType),
    /// The `void` type (only used in method return types).
    Void(Span),
}

impl Type {
    pub fn span(&self) -> Span {
        match self {
            Self::Primitive(p) => p.span(),
            Self::Reference(r) => r.span(),
            Self::Void(s) => *s,
        }
    }

    /// Check if this is the void type.
    pub fn is_void(&self) -> bool {
        matches!(self, Self::Void(_))
    }

    /// Check if this is a primitive type.
    pub fn is_primitive(&self) -> bool {
        matches!(self, Self::Primitive(_))
    }
}

/// A primitive Java type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
    Boolean,
}

impl PrimitiveType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Byte => "byte",
            Self::Short => "short",
            Self::Int => "int",
            Self::Long => "long",
            Self::Char => "char",
            Self::Float => "float",
            Self::Double => "double",
            Self::Boolean => "boolean",
        }
    }

    pub fn span(&self) -> Span {
        Span::call_site() // Primitive types are keywords, spans are set at the Type level
    }
}

/// A reference type wrapper (class/interface type, type variable, or array type).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReferenceType {
    /// A class or interface type: `String`, `List<Integer>`, `java.util.Map.Entry`.
    ClassOrInterfaceType(ClassOrInterfaceType),
    /// A type variable: `T`.
    TypeVar(Ident),
    /// An array type: `int[]`, `String[][]`.
    Array(ArrayType),
}

impl ReferenceType {
    pub fn span(&self) -> Span {
        match self {
            Self::ClassOrInterfaceType(t) => t.span(),
            Self::TypeVar(i) => i.span(),
            Self::Array(a) => a.span,
        }
    }
}

/// A class or interface type with optional type arguments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassOrInterfaceType {
    /// For a simple type like `List`, this is the name with optional type args.
    /// For a qualified type like `Map.Entry`, this contains both segments.
    pub path: Path,
    pub annotations_prefix: Vec<Annotation>, // annotations before the type (rare)
}

impl ClassOrInterfaceType {
    pub fn span(&self) -> Span {
        self.path.span
    }

    pub fn name(&self) -> &Ident {
        self.path.last_ident()
    }

    pub fn type_args(&self) -> Option<&TypeArguments> {
        self.path.last_segment().args.as_ref()
    }
}

/// An array type with its dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub elem_type: Box<Type>,
    pub dims: Vec<ArrayDim>,
    pub span: Span,
}

/// A single array dimension `[]` with optional annotations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayDim {
    pub bracket_span: (Span, Span),
    pub annotations: Vec<Annotation>,
}
