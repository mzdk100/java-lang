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
            Type::Primitive(p) => p.span(),
            Type::Reference(r) => r.span(),
            Type::Void(s) => *s,
        }
    }

    /// Check if this is the void type.
    pub fn is_void(&self) -> bool {
        matches!(self, Type::Void(_))
    }

    /// Check if this is a primitive type.
    pub fn is_primitive(&self) -> bool {
        matches!(self, Type::Primitive(_))
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
            PrimitiveType::Byte => "byte",
            PrimitiveType::Short => "short",
            PrimitiveType::Int => "int",
            PrimitiveType::Long => "long",
            PrimitiveType::Char => "char",
            PrimitiveType::Float => "float",
            PrimitiveType::Double => "double",
            PrimitiveType::Boolean => "boolean",
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
            ReferenceType::ClassOrInterfaceType(t) => t.span(),
            ReferenceType::TypeVar(i) => i.span(),
            ReferenceType::Array(a) => a.span,
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
