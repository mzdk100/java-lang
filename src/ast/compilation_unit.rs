//! Compilation unit (top-level Java file structure).

use crate::{ast::path::Path, ident::Ident, span::Span};

use super::{
    Comment,
    attribute::Annotation,
    item::{ClassBodyDecl, TypeDecl},
};

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
    /// The implicitly declared class, if this is a compact compilation unit
    pub implicit_class: Option<ImplicitClassDecl>,
    /// All comments in the source file.
    pub comments: Vec<Comment>,
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
                        None => match &self.implicit_class {
                            Some(c) => c.span(),
                            None => Span::call_site(),
                        },
                    },
                },
            },
        };
        let end = match self.type_decls.last() {
            Some(decl) => decl.span(),
            None => match self.module {
                Some(ref m) => m.span(),
                None => match &self.implicit_class {
                    Some(c) => c.span(),
                    None => match self.imports.last() {
                        Some(imp) => imp.span(),
                        None => start,
                    },
                },
            },
        };
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
