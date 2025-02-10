use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub struct CompilationUnitDeclaration<'a> {
    pub package: PackageDeclaration<'a>,
    pub imports: Vec<ImportDeclaration<'a>>,
    pub items: Vec<CompilationUnitItemDeclaration<'a>>,
}

impl<'a> CompilationUnitDeclaration<'a> {
    pub(crate) fn new(
        package: PackageDeclaration<'a>,
        imports: Vec<ImportDeclaration<'a>>,
        items: Vec<CompilationUnitItemDeclaration<'a>>,
    ) -> Self {
        Self {
            package,
            imports,
            items,
        }
    }
}

impl<'a> Display for CompilationUnitDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.package, f)?;
        for i in &self.imports {
            Display::fmt(&i, f)?;
            write!(f, "\n")?;
        }
        for i in &self.items {
            Display::fmt(&i, f)?;
            write!(f, "\n")?;
        }

        Ok(())
    }
}

/// ImportDeclaration 枚举表示Java中的导入声明。
#[derive(Debug)]
pub enum ImportDeclaration<'a> {
    /// 单类型导入声明，参数是导入的类或接口的名称。
    SimpleType(Cow<'a, str>),
    /// 需求类型导入声明，参数是导入的包、类或接口的名称（路径）。
    TypeOnDemand(Cow<'a, str>),
    /// 单静态导入声明，参数是导入的类或接口的名称 + 导入的静态成员的名称。
    SingleStatic(Cow<'a, str>),
    /// 静态需求导入声明，参数是导入的类或接口的名称。
    StaticOnDemand(Cow<'a, str>),
}

impl<'a> Display for ImportDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::SimpleType(r) | Self::SingleStatic(r) => write!(f, "import {};", r),
            Self::TypeOnDemand(r) | Self::StaticOnDemand(r) => write!(f, "import {}.*;", r),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct PackageDeclaration<'a> {
    pub name: Cow<'a, str>,
    pub modifiers: Vec<Annotation<'a>>,
    pub documentation: Cow<'a, str>,
}

impl<'a> PackageDeclaration<'a> {
    pub(crate) fn new(
        name: Cow<'a, str>,
        modifiers: Vec<Annotation<'a>>,
        documentation: Cow<'a, str>,
    ) -> Self {
        Self {
            name,
            modifiers,
            documentation,
        }
    }
}

impl<'a> Display for PackageDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if !self.documentation.is_empty() {
            write!(f, "/**{}*/\n", self.documentation)?;
        }
        for i in &self.modifiers {
            Display::fmt(&i, f)?;
        }
        if !self.name.is_empty() {
            write!(f, "package {};\n", self.name)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum CompilationUnitItemDeclaration<'a> {
    Class(ClassDeclaration<'a>),
    Enum(EnumDeclaration<'a>),
    Interface(InterfaceDeclaration<'a>),
    Annotation(AnnotationDeclaration<'a>),
}

impl<'a> Display for CompilationUnitItemDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Class(r) => Display::fmt(r, f),
            Self::Interface(r) => Display::fmt(r, f),
            Self::Enum(r) => Display::fmt(r, f),
            Self::Annotation(r) => Display::fmt(r, f),
        }
    }
}

#[derive(Debug)]
pub struct ClassDeclaration<'a> {
    name: Cow<'a, str>,
    // attrs = ("type_parameters", "extends", "implements")
    // attrs = ("body", "documentation",, "modifiers", "annotations")
    // fields, methods, constructors
}

impl<'a> Display for ClassDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "class {}", self.name)
    }
}

#[derive(Debug)]
pub struct EnumDeclaration<'a> {
    name: Cow<'a, str>,
    // attrs = ("implements",)
    // fields, methods
    // attrs = ("body", "documentation",, "modifiers", "annotations")
    // fields, methods, constructors
}

impl<'a> Display for EnumDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "enum {}", self.name)
    }
}

#[derive(Debug)]
pub struct InterfaceDeclaration<'a> {
    name: Cow<'a, str>,
    // attrs = ("type_parameters", "extends",)
    // attrs = ("body", "documentation",, "modifiers", "annotations")
    // fields, methods, constructors
}

impl<'a> Display for InterfaceDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "interface {}", self.name)
    }
}

#[derive(Debug)]
pub struct AnnotationDeclaration<'a> {
    name: Cow<'a, str>,
    // attrs = ("body", "documentation",, "modifiers", "annotations")
    // fields, methods, constructors
}

impl<'a> Display for AnnotationDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "@interface {}", self.name)
    }
}

pub trait BasicType {
    // attrs = ("name", "dimensions",)
}

pub trait ReferenceType {
    // attrs = ("arguments", "sub_type", "name", "dimensions",)
}

pub trait TypeArgument {
    // attrs = ("type", "pattern_type")
}

pub trait TypeParameter {
    // attrs = ("name", "extends")
}

#[derive(Debug, PartialEq)]
pub struct Annotation<'a> {
    pub name: Cow<'a, str>,
    // element:
}

impl<'a> Display for Annotation<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "@{}", self.name)
    }
}

pub trait ElementValuePair {
    // attrs = ("name", "value")
}

pub trait ElementArrayValue {
    // attrs = ("values",)
}

/// member
pub trait MethodDeclaration {
    // attrs = ("type_parameters", "return_type", "name", "parameters", "throws", "body", "modifiers", "annotations", "documentation",)
}

/// member
pub trait FieldDeclaration {
    // attrs = ("type", "declarators", "modifiers", "annotations", "documentation",)
}

pub trait ConstructorDeclaration {
    // attrs = ("type_parameters", "name", "parameters", "throws", "body", "documentation",, "modifiers", "annotations")
}

pub trait ConstantDeclaration: FieldDeclaration {}

pub trait ArrayInitializer {
    // attrs = ("initializers",)
}

pub trait VariableDeclaration {
    // attrs = ("type", "declarators", "modifiers", "annotations")
}

pub trait LocalVariableDeclaration: VariableDeclaration {}

pub trait VariableDeclarator {
    // attrs = ("name", "dimensions", "initializer")
}

pub trait FormalParameter {
    // attrs = ("type", "name", "varargs", "modifiers", "annotations")
}

pub trait InferredFormalParameter {
    // attrs = ('name',)
}

pub trait IfStatement {
    // attrs = ("condition", "then_statement", "else_statement", "label",)
}

pub trait WhileStatement {
    // attrs = ("condition", "body", "label",)
}

pub trait DoStatement {
    // attrs = ("condition", "body", "label",)
}

pub trait ForStatement {
    // attrs = ("control", "body", "label",)
}

pub trait AssertStatement {
    // attrs = ("condition", "value", "label",)
}

pub trait BreakStatement {
    // attrs = ("goto",, "label",)
}

pub trait ContinueStatement {
    // attrs = ("goto",, "label",)
}

pub trait ReturnStatement {
    // attrs = ("expression",, "label",)
}

pub trait ThrowStatement {
    // attrs = ("expression",, "label",)
}

pub trait SynchronizedStatement {
    // attrs = ("lock", "block", "label",)
}

pub trait TryStatement {
    // attrs = ("resources", "block", "catches", "finally_block", "label",)
}

pub trait SwitchStatement {
    // attrs = ("expression", "cases", "label",)
}

pub trait BlockStatement {
    // attrs = ("statements",, "label",)
}

pub trait StatementExpression {
    // attrs = ("expression",, "label",)
}

pub trait TryResource {
    // attrs = ("type", "name", "value", "modifiers", "annotations")
}

pub trait CatchClause {
    // attrs = ("parameter", "block", "label",)
}

pub trait CatchClauseParameter {
    // attrs = ("types", "name", "modifiers", "annotations")
}

pub trait SwitchStatementCase {
    // attrs = ("case", "statements")
}

pub trait ForControl {
    // attrs = ("init", "condition", "update")
}

pub trait EnhancedForControl {
    // attrs = ("var", "iterable")
}

pub trait Expression {}

pub trait Assignment: Expression {
    // attrs = ("expressionl", "value", "type")
}

pub trait TernaryExpression: Expression {
    // attrs = ("condition", "if_true", "if_false")
}

pub trait BinaryOperation: Expression {
    // attrs = ("operator", "operandl", "operandr")
}

pub trait Cast: Expression {
    // attrs = ("type", "expression")
}

pub trait MethodReference: Expression {
    // attrs = ("expression", "method", "type_arguments")
}

pub trait LambdaExpression: Expression {
    // attrs = ('parameters', 'body')
}

pub trait Primary: Expression {
    // attrs = ("prefix_operators", "postfix_operators", "qualifier", "selectors")
}

pub trait Literal: Primary {
    // attrs = ("value",)
}

pub trait This: Primary {}

pub trait MemberReference: Primary {
    // attrs = ("member",)
}

pub trait Invocation: Primary {
    // attrs = ("type_arguments", "arguments")
}

pub trait ExplicitConstructorInvocation: Invocation {}

pub trait SuperConstructorInvocation: Invocation {}

pub trait MethodInvocation: Invocation {
    // attrs = ("member",)
}

pub trait SuperMethodInvocation: Invocation {
    // attrs = ("member",)
}

pub trait SuperMemberReference: Primary {
    // attrs = ("member",)
}

pub trait ArraySelector: Expression {
    // attrs = ("index",)
}

pub trait ClassReference: Primary {
    // attrs = ("type",)
}

pub trait VoidClassReference: ClassReference {}

pub trait Creator: Primary {
    // attrs = ("type",)
}

pub trait ArrayCreator: Creator {
    // attrs = ("dimensions", "initializer")
}

pub trait ClassCreator: Creator {
    // attrs = ("constructor_type_arguments", "arguments", "body")
}

pub trait InnerClassCreator: Creator {
    // attrs = ("constructor_type_arguments", "arguments", "body")
}

pub trait EnumBody {
    // attrs = ("constants", "declarations")
}

pub trait EnumConstantDeclaration {
    // attrs = ("name", "arguments", "body", "documentation",, "modifiers", "annotations")
}

pub trait AnnotationMethod {
    // attrs = ("name", "return_type", "dimensions", "default", "modifiers", "annotations")
    fn name(&self) {}
}
