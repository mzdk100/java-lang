//! Compilation unit (top-level Java file structure).

use crate::{ast::path::Path, ident::Ident, span::Span};

use super::{attribute::Annotation, item::TypeDecl};

/// A Java compilation unit (a single source file).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompilationUnit {
    /// The package declaration, if present.
    pub package: Option<PackageDecl>,
    /// Import declarations.
    pub imports: Vec<ImportDecl>,
    /// Top-level type declarations.
    pub type_decls: Vec<TypeDecl>,
    /// The module declaration, if present (for module-info.java).
    pub module: Option<super::item::ModuleDecl>,
}

impl CompilationUnit {
    /// Get the span of the entire compilation unit.
    pub fn span(&self) -> Span {
        let start = match &self.package {
            Some(pkg) => pkg.span,
            None => match self.imports.first() {
                Some(imp) => imp.span(),
                None => match self.type_decls.first() {
                    Some(decl) => decl.span(),
                    None => match &self.module {
                        Some(m) => m.span(),
                        None => Span::call_site(),
                    },
                },
            },
        };
        let end = match self.type_decls.last() {
            Some(decl) => decl.span(),
            None => match self.module {
                Some(ref m) => m.span(),
                None => match self.imports.last() {
                    Some(imp) => imp.span(),
                    None => start,
                },
            },
        };
        start.join(end)
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
}

impl ImportDecl {
    pub fn span(&self) -> Span {
        match self {
            ImportDecl::SingleType {
                import_span,
                semi_span,
                ..
            }
            | ImportDecl::TypeOnDemand {
                import_span,
                semi_span,
                ..
            }
            | ImportDecl::SingleStatic {
                import_span,
                semi_span,
                ..
            }
            | ImportDecl::StaticOnDemand {
                import_span,
                semi_span,
                ..
            } => import_span.join(semi_span),
        }
    }
}
