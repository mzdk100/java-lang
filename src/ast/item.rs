//! Item types (class, interface, enum, record, module declarations).

use crate::{ident::Ident, span::Span};

use super::{
    attribute::Annotation,
    expr::Expr,
    generics::TypeParameters,
    stmt::{Block, Stmt},
    ty::Type,
};

/// A top-level type declaration or semicolon.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeDecl {
    Class(ClassDecl),
    Interface(InterfaceDecl),
    Enum(EnumDecl),
    Record(RecordDecl),
    AnnotationType(AnnotationInterfaceDecl),
    /// A standalone semicolon in the top level.
    Empty(Span),
}

impl TypeDecl {
    pub fn span(&self) -> Span {
        match self {
            TypeDecl::Class(c) => c.span(),
            TypeDecl::Interface(i) => i.span(),
            TypeDecl::Enum(e) => e.span(),
            TypeDecl::Record(r) => r.span(),
            TypeDecl::AnnotationType(a) => a.span(),
            TypeDecl::Empty(s) => *s,
        }
    }
}

/// A class declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassDecl {
    pub modifiers: Vec<Modifier>,
    pub class_span: Span,
    pub name: Ident,
    pub type_params: Option<TypeParameters>,
    pub extends_clause: Option<ExtendsClause>,
    pub implements_clause: Option<ImplementsClause>,
    pub permits_clause: Option<PermitsClause>,
    pub body: ClassBodyDeclList,
}

impl ClassDecl {
    pub fn span(&self) -> Span {
        self.modifiers
            .first()
            .map_or(self.class_span, |m| m.span())
            .join(self.body.brace_span.1)
    }
}

/// An interface declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceDecl {
    pub modifiers: Vec<Modifier>,
    pub interface_span: Span,
    pub name: Ident,
    pub type_params: Option<TypeParameters>,
    pub extends_clause: Option<InterfaceExtendsClause>,
    pub permits_clause: Option<PermitsClause>,
    pub body: InterfaceBody,
}

impl InterfaceDecl {
    pub fn span(&self) -> Span {
        self.modifiers
            .first()
            .map_or(self.interface_span, |m| m.span())
            .join(self.body.brace_span.1)
    }
}

/// An enum declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumDecl {
    pub modifiers: Vec<Modifier>,
    pub enum_span: Span,
    pub name: Ident,
    pub implements_clause: Option<ImplementsClause>,
    pub body: EnumBody,
}

impl EnumDecl {
    pub fn span(&self) -> Span {
        self.modifiers
            .first()
            .map_or(self.enum_span, |m| m.span())
            .join(self.body.brace_span.1)
    }
}

/// A record declaration (Java 16+).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordDecl {
    pub modifiers: Vec<Modifier>,
    pub record_span: Span,
    pub name: Ident,
    pub type_params: Option<TypeParameters>,
    pub components: RecordComponents,
    pub implements_clause: Option<ImplementsClause>,
    pub body: RecordBody,
}

impl RecordDecl {
    pub fn span(&self) -> Span {
        self.modifiers
            .first()
            .map_or(self.record_span, |m| m.span())
            .join(self.body.brace_span.1)
    }
}

/// A module declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleDecl {
    pub annotations: Vec<Annotation>,
    pub open_span: Option<Span>,
    pub module_span: Span,
    pub name: crate::ast::path::Path,
    pub brace_span: (Span, Span),
    pub directives: Vec<ModuleDirective>,
}

impl ModuleDecl {
    pub fn span(&self) -> Span {
        self.annotations
            .first()
            .map_or(self.module_span, |a| a.span())
            .join(self.brace_span.1)
    }
}

/// A module directive.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleDirective {
    Requires {
        requires_span: Span,
        modifiers: Vec<RequiresModifier>,
        module: crate::ast::path::Path,
        semi_span: Span,
    },
    Exports {
        exports_span: Span,
        pkg: crate::ast::path::Path,
        to_modules: Option<(Span, Vec<crate::ast::path::Path>)>,
        semi_span: Span,
    },
    Opens {
        opens_span: Span,
        pkg: crate::ast::path::Path,
        to_modules: Option<(Span, Vec<crate::ast::path::Path>)>,
        semi_span: Span,
    },
    Uses {
        uses_span: Span,
        ty: Type,
        semi_span: Span,
    },
    Provides {
        provides_span: Span,
        ty: Type,
        with_span: Span,
        impls: Vec<Type>,
        semi_span: Span,
    },
}

/// Requires modifier: `transitive`, `static`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequiresModifier {
    Transitive(Span),
    Static(Span),
}

/// A modifier (keyword or annotation).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Modifier {
    /// `public`
    Public(Span),
    /// `protected`
    Protected(Span),
    /// `private`
    Private(Span),
    /// `static`
    Static(Span),
    /// `abstract`
    Abstract(Span),
    /// `final`
    Final(Span),
    /// `synchronized`
    Synchronized(Span),
    /// `native`
    Native(Span),
    /// `strictfp`
    Strictfp(Span),
    /// `transient`
    Transient(Span),
    /// `volatile`
    Volatile(Span),
    /// `default`
    Default(Span),
    /// `sealed`
    Sealed(Span),
    /// `non-sealed`
    NonSealed(Span),
    /// An annotation
    Annotation(Annotation),
}

impl Modifier {
    pub fn span(&self) -> Span {
        match self {
            Modifier::Public(s)
            | Modifier::Protected(s)
            | Modifier::Private(s)
            | Modifier::Static(s)
            | Modifier::Abstract(s)
            | Modifier::Final(s)
            | Modifier::Synchronized(s)
            | Modifier::Native(s)
            | Modifier::Strictfp(s)
            | Modifier::Transient(s)
            | Modifier::Volatile(s)
            | Modifier::Default(s)
            | Modifier::Sealed(s)
            | Modifier::NonSealed(s) => *s,
            Modifier::Annotation(a) => a.span(),
        }
    }

    /// Check if this modifier is an access modifier.
    pub fn is_access_modifier(&self) -> bool {
        matches!(
            self,
            Modifier::Public(_) | Modifier::Protected(_) | Modifier::Private(_)
        )
    }

    /// Check if this modifier is an annotation.
    pub fn is_annotation(&self) -> bool {
        matches!(self, Modifier::Annotation(_))
    }
}

/// An `extends` clause: `extends SuperClass`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtendsClause {
    pub extends_span: Span,
    pub supertype: Type,
}

/// An `implements` clause: `implements Interface1, Interface2`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementsClause {
    pub implements_span: Span,
    pub supertypes: Vec<Type>,
}

/// An interface extends clause: `extends Interface1, Interface2`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceExtendsClause {
    pub extends_span: Span,
    pub supertypes: Vec<Type>,
}

/// A `permits` clause: `permits Sub1, Sub2`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PermitsClause {
    pub permits_span: Span,
    pub types: Vec<Type>,
}

/// Class body: `{ members }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassBodyDeclList {
    pub brace_span: (Span, Span),
    pub declarations: Vec<ClassBodyDecl>,
}

/// A declaration inside a class body.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassBodyDecl {
    /// A field declaration.
    Field(FieldDecl),
    /// A method declaration.
    Method(MethodDecl),
    /// A constructor declaration.
    Constructor(ConstructorDecl),
    /// A static initializer block: `static { ... }`.
    StaticInit(StaticInit),
    /// An instance initializer block: `{ ... }`.
    InstanceInit(InstanceInit),
    /// A nested class declaration.
    Class(ClassDecl),
    /// A nested interface declaration.
    Interface(InterfaceDecl),
    /// A nested enum declaration.
    Enum(EnumDecl),
    /// A nested record declaration.
    Record(RecordDecl),
    /// A nested annotation type declaration.
    AnnotationType(AnnotationInterfaceDecl),
    /// A semicolon (empty declaration).
    Empty(Span),
}

impl ClassBodyDecl {
    pub fn span(&self) -> Span {
        match self {
            ClassBodyDecl::Field(f) => f.span(),
            ClassBodyDecl::Method(m) => m.span(),
            ClassBodyDecl::Constructor(c) => c.span(),
            ClassBodyDecl::StaticInit(s) => s.span(),
            ClassBodyDecl::InstanceInit(i) => i.span(),
            ClassBodyDecl::Class(c) => c.span(),
            ClassBodyDecl::Interface(i) => i.span(),
            ClassBodyDecl::Enum(e) => e.span(),
            ClassBodyDecl::Record(r) => r.span(),
            ClassBodyDecl::AnnotationType(a) => a.span(),
            ClassBodyDecl::Empty(s) => *s,
        }
    }
}

/// A field declaration: `[modifiers] type name [= expr] [, name [= expr]] ;`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldDecl {
    pub modifiers: Vec<Modifier>,
    pub ty: Type,
    pub declarators: Vec<super::stmt::VariableDeclarator>,
    pub semi_span: Span,
}

impl FieldDecl {
    pub fn span(&self) -> Span {
        self.semi_span
    }
}

/// A method declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodDecl {
    pub modifiers: Vec<Modifier>,
    pub type_params: Option<TypeParameters>,
    pub return_type: MethodReturnType,
    pub name: Ident,
    pub receiver_param: Option<ReceiverParameter>,
    pub params: Vec<FormalParameter>,
    pub paren_span: (Span, Span),
    pub throws_clause: Option<ThrowsClause>,
    pub body: Option<Block>,
}

impl MethodDecl {
    pub fn span(&self) -> Span {
        let end = match &self.body {
            Some(b) => b.brace_span.1,
            None => self.paren_span.1,
        };
        self.modifiers
            .first()
            .map_or(self.name.span(), |m| m.span())
            .join(end)
    }
}

/// The return type of a method.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodReturnType {
    /// A declared return type.
    Type(Type),
    /// `void`
    Void(Span),
}

/// A receiver parameter: `Type [name.] this`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReceiverParameter {
    pub annotations: Vec<Annotation>,
    pub ty: Type,
    pub name: Option<Ident>,
    pub dot_span: Option<Span>,
    pub this_span: Span,
}

/// A formal parameter of a method or constructor.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FormalParameter {
    /// A normal parameter: `[final] Type name`.
    Normal {
        modifiers: Vec<Modifier>,
        ty: Type,
        name: Option<Ident>, // None for unnamed (Java 21+)
    },
    /// A varargs parameter: `Type... name`.
    VarArgs {
        modifiers: Vec<Modifier>,
        ty: Type,
        ellipsis_span: Span,
        name: Option<Ident>,
    },
}

/// A throws clause: `throws Type1, Type2`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrowsClause {
    pub throws_span: Span,
    pub types: Vec<Type>,
}

/// A constructor declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructorDecl {
    pub modifiers: Vec<Modifier>,
    pub type_params: Option<TypeParameters>,
    pub name: Ident,
    pub receiver_param: Option<ReceiverParameter>,
    pub params: Vec<FormalParameter>,
    pub paren_span: (Span, Span),
    pub throws_clause: Option<ThrowsClause>,
    pub body: ConstructorBody,
}

impl ConstructorDecl {
    pub fn span(&self) -> Span {
        self.modifiers
            .first()
            .map_or(self.name.span(), |m| m.span())
            .join(self.body.brace_span.1)
    }
}

/// A constructor body.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructorBody {
    pub brace_span: (Span, Span),
    pub explicit_constructor_call: Option<ExplicitConstructorCall>,
    pub stmts: Vec<Stmt>,
}

/// An explicit constructor invocation: `this(args)` or `super(args)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExplicitConstructorCall {
    This {
        type_args: Option<crate::ast::path::TypeArguments>,
        this_span: Span,
        paren_span: (Span, Span),
        args: Vec<Expr>,
        semi_span: Span,
    },
    Super {
        type_args: Option<crate::ast::path::TypeArguments>,
        super_span: Span,
        paren_span: (Span, Span),
        args: Vec<Expr>,
        semi_span: Span,
    },
    SuperFromExpr {
        target: Box<Expr>,
        dot_span: Span,
        type_args: Option<crate::ast::path::TypeArguments>,
        super_span: Span,
        paren_span: (Span, Span),
        args: Vec<Expr>,
        semi_span: Span,
    },
}

/// A static initializer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticInit {
    pub static_span: Span,
    pub block: Block,
}

impl StaticInit {
    pub fn span(&self) -> Span {
        self.static_span.join(self.block.brace_span.1)
    }
}

/// An instance initializer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstanceInit {
    pub block: Block,
}

impl InstanceInit {
    pub fn span(&self) -> Span {
        self.block.span()
    }
}

/// An interface body: `{ members }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceBody {
    pub brace_span: (Span, Span),
    pub members: Vec<InterfaceMemberDecl>,
}

/// A member declaration inside an interface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterfaceMemberDecl {
    Field(FieldDecl),
    Method(MethodDecl),
    Class(ClassDecl),
    Interface(InterfaceDecl),
    Enum(EnumDecl),
    Record(RecordDecl),
    AnnotationInterface(AnnotationInterfaceDecl),
    Empty(Span),
}

/// An annotation interface declaration: `@interface Name { ... }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnnotationInterfaceDecl {
    pub modifiers: Vec<Modifier>,
    pub at_span: Span,
    pub interface_span: Span,
    pub name: Ident,
    pub body: AnnotationInterfaceBody,
}

impl AnnotationInterfaceDecl {
    pub fn span(&self) -> Span {
        self.modifiers
            .first()
            .map_or(self.at_span, |m| m.span())
            .join(self.body.brace_span.1)
    }
}

/// Annotation interface body.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnnotationInterfaceBody {
    pub brace_span: (Span, Span),
    pub members: Vec<AnnotationInterfaceMember>,
}

/// A member of an annotation interface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnnotationInterfaceMember {
    /// An annotation element: `Type name() [default value];`
    Element(AnnotationElement),
    /// A constant field.
    Field(FieldDecl),
    /// A nested class.
    Class(ClassDecl),
    /// A nested interface.
    Interface(InterfaceDecl),
    /// A nested enum.
    Enum(EnumDecl),
    /// A nested record.
    Record(RecordDecl),
    /// A nested annotation interface.
    AnnotationInterface(AnnotationInterfaceDecl),
    Empty(Span),
}

/// An annotation element declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnnotationElement {
    pub modifiers: Vec<Modifier>,
    pub ty: Type,
    pub name: Ident,
    pub paren_span: (Span, Span),
    pub dims: Vec<super::ty::ArrayDim>,
    pub default_value: Option<(Span, super::attribute::ElementValue)>,
    pub semi_span: Span,
}

/// Enum body: `{ constants [,] [members] }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumBody {
    pub brace_span: (Span, Span),
    pub constants: Vec<EnumConstant>,
    pub comma_span: Option<Span>,
    pub members: Vec<ClassBodyDecl>,
}

/// An enum constant: `NAME[(args)] [{ class body }]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumConstant {
    pub annotations: Vec<Annotation>,
    pub name: Ident,
    pub paren_span: Option<(Span, Span)>,
    pub args: Vec<Expr>,
    pub body: Option<ClassBodyDeclList>,
}

/// Record components: `(Type name, Type name, ...)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordComponents {
    pub paren_span: (Span, Span),
    pub components: Vec<RecordComponent>,
}

/// A single record component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecordComponent {
    Normal {
        annotations: Vec<Annotation>,
        ty: Type,
        name: Ident,
    },
    VarArgs {
        annotations: Vec<Annotation>,
        ty: Type,
        ellipsis_span: Span,
        name: Ident,
    },
}

/// Record body: `{ members }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordBody {
    pub brace_span: (Span, Span),
    pub members: Vec<RecordBodyDecl>,
}

/// A member of a record body.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecordBodyDecl {
    Field(FieldDecl),
    Method(MethodDecl),
    Constructor(ConstructorDecl),
    InstanceInit(InstanceInit),
    StaticInit(StaticInit),
    Class(ClassDecl),
    Interface(InterfaceDecl),
    Enum(EnumDecl),
    Record(RecordDecl),
    /// A compact constructor: `public Name { ... }`.
    CompactConstructor(CompactConstructorDecl),
    Empty(Span),
}

/// A compact constructor declaration for records.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompactConstructorDecl {
    pub modifiers: Vec<Modifier>,
    pub name: Ident,
    pub body: ConstructorBody,
}
