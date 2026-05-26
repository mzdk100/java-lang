//! Path types.

use std::fmt;

use crate::{ident::Ident, span::Span};

use super::ty::Type;

/// A qualified path like `java.util.List` or just `List`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

impl Path {
    /// Create a path from a single identifier.
    pub fn from_ident(ident: Ident) -> Self {
        let span = ident.span();
        Path {
            segments: vec![PathSegment { ident, args: None }],
            span,
        }
    }

    /// Check if this path is a simple identifier (single segment, no type args).
    pub fn is_ident(&self) -> bool {
        self.segments.len() == 1 && self.segments[0].args.is_none()
    }

    /// Get the last identifier in the path.
    pub fn last_ident(&self) -> &Ident {
        &self.segments.last().unwrap().ident
    }

    /// Get the last segment of the path.
    pub fn last_segment(&self) -> &PathSegment {
        self.segments.last().unwrap()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, seg) in self.segments.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", seg.ident)?;
            if let Some(args) = &seg.args {
                write!(f, "<")?;
                for (j, arg) in args.args.iter().enumerate() {
                    if j > 0 {
                        write!(f, ", ")?;
                    }
                    match arg {
                        TypeArgument::Type(ty) => write!(f, "{:?}", ty)?,
                        TypeArgument::Wildcard(w) => write!(f, "{:?}", w)?,
                    }
                }
                write!(f, ">")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for PathSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident)?;
        if let Some(args) = &self.args {
            write!(f, "<")?;
            for (j, arg) in args.args.iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                match arg {
                    TypeArgument::Type(ty) => write!(f, "{:?}", ty)?,
                    TypeArgument::Wildcard(w) => write!(f, "{:?}", w)?,
                }
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

/// A segment of a path, optionally with type arguments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSegment {
    pub ident: Ident,
    pub args: Option<TypeArguments>,
}

/// Type arguments: `<String, Integer>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArguments {
    pub args: Vec<TypeArgument>,
    pub lt_token: Span,
    pub gt_tokens: Vec<Span>, // Multiple > tokens for nested generics
}

/// A type argument, which can be a concrete type or a wildcard.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeArgument {
    /// A specific type: `String`, `List<Integer>`.
    Type(Box<Type>),
    /// A wildcard: `?`, `? extends Number`, `? super String`.
    Wildcard(Wildcard),
}

/// A wildcard type argument: `? extends T` or `? super T`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wildcard {
    pub bound: Option<WildcardBound>,
    pub span: Span,
}

/// The bound of a wildcard type argument.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WildcardBound {
    Extends(Box<Type>),
    Super(Box<Type>),
}
