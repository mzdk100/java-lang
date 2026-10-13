//! Compilation unit (top-level Java file structure).

use crate::{ast::path::Path, ident::Ident, span::Span};

use super::{
    Comment,
    attribute::Annotation,
    item::{ClassBodyDecl, ModuleDecl, TypeDecl},
};

/// A Java compilation unit (a single source file).
///
/// Per JLS §7.3, a compilation unit is exactly one of an ordinary, modular,
/// or compact compilation unit; `kind` encodes which.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompilationUnit {
    /// Import declarations.
    pub imports: Vec<ImportDecl>,
    /// The ordinary/modular/compact body of the compilation unit.
    pub kind: CompilationUnitKind,
    /// All comments in the source file.
    pub comments: Vec<Comment>,
}

/// The three mutually exclusive forms of compilation unit (JLS §7.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompilationUnitKind {
    /// An `OrdinaryCompilationUnit`: an optional package declaration followed
    /// by top-level type declarations.
    Ordinary {
        package: Option<PackageDecl>,
        type_decls: Vec<TypeDecl>,
    },
    /// A `ModularCompilationUnit` (`module-info.java`).
    Modular(ModuleDecl),
    /// A `CompactCompilationUnit` (JLS §8.1.8): top-level methods/fields/nested
    /// types with no enclosing `class` declaration.
    Compact(ImplicitClassDecl),
}

impl CompilationUnit {
    /// The package declaration, if this is an ordinary compilation unit that has one.
    pub fn package(&self) -> Option<&PackageDecl> {
        match &self.kind {
            CompilationUnitKind::Ordinary { package, .. } => package.as_ref(),
            CompilationUnitKind::Modular(_) | CompilationUnitKind::Compact(_) => None,
        }
    }

    /// The top-level type declarations, if this is an ordinary compilation unit.
    /// Empty for modular and compact compilation units.
    pub fn type_decls(&self) -> &[TypeDecl] {
        match &self.kind {
            CompilationUnitKind::Ordinary { type_decls, .. } => type_decls,
            CompilationUnitKind::Modular(_) | CompilationUnitKind::Compact(_) => &[],
        }
    }

    /// Get the span of the entire compilation unit.
    pub fn span(&self) -> Span {
        let kind_span = match &self.kind {
            CompilationUnitKind::Modular(m) => Some(m.span()),
            CompilationUnitKind::Compact(c) => Some(c.span()),
            CompilationUnitKind::Ordinary { .. } => None,
        };

        let start = self
            .package()
            .map(|pkg| pkg.span)
            .or_else(|| self.imports.first().map(ImportDecl::span))
            .or_else(|| self.type_decls().first().map(TypeDecl::span))
            .or(kind_span)
            .unwrap_or_else(Span::call_site);

        let end = self
            .type_decls()
            .last()
            .map(TypeDecl::span)
            .or(kind_span)
            .or_else(|| self.imports.last().map(ImportDecl::span))
            .unwrap_or(start);

        start.join(end)
    }
}

/// The class implicitly declared by a compact compilation unit (JLS §8.1.8):
/// top-level methods/fields/nested types with no enclosing `class` declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplicitClassDecl {
    pub members: Vec<ClassBodyDecl>,
}

impl ImplicitClassDecl {
    pub fn span(&self) -> Span {
        match (self.members.first(), self.members.last()) {
            (Some(first), Some(last)) => first.span().join(last.span()),
            _ => Span::call_site(),
        }
    }
}

/// A package declaration: `package com.example.foo;`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageDecl {
    pub annotations: Vec<Annotation>,
    pub package_span: Span,
    pub name: Path,
    pub semi_span: Span,
    pub span: Span,
}

/// An import declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImportDecl {
    /// Single type import: `import java.util.List;`
    SingleType {
        import_span: Span,
        path: Path,
        semi_span: Span,
    },
    /// Type import on demand: `import java.util.*;`
    TypeOnDemand {
        import_span: Span,
        path: Path,
        star_span: Span,
        semi_span: Span,
    },
    /// Single static import: `import static java.util.Collections.sort;`
    SingleStatic {
        import_span: Span,
        static_span: Span,
        path: Path,
        member: Ident,
        semi_span: Span,
    },
    /// Static import on demand: `import static java.util.Collections.*;`
    StaticOnDemand {
        import_span: Span,
        static_span: Span,
        path: Path,
        star_span: Span,
        semi_span: Span,
    },
    /// Module import: `import module org.junit.jupiter.api;`
    Module {
        import_span: Span,
        module_span: Span,
        name: Path,
        semi_span: Span,
    },
}

impl ImportDecl {
    pub fn span(&self) -> Span {
        match self {
            Self::SingleType {
                import_span,
                semi_span,
                ..
            }
            | Self::TypeOnDemand {
                import_span,
                semi_span,
                ..
            }
            | Self::SingleStatic {
                import_span,
                semi_span,
                ..
            }
            | Self::StaticOnDemand {
                import_span,
                semi_span,
                ..
            }
            | Self::Module {
                import_span,
                semi_span,
                ..
            } => import_span.join(semi_span),
        }
    }
}
