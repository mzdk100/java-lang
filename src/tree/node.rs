use std::borrow::Cow;

#[derive(Debug)]
pub struct CompilationUnit<'a> {
    pub package: PackageDeclaration<'a>,
    pub imports: Vec<Import<'a>>,
    pub items: Vec<CompilationUnitItem>,
}

impl<'a> CompilationUnit<'a> {
    pub(crate) fn new(package: PackageDeclaration<'a>, imports: Vec<Import<'a>>, items: Vec<CompilationUnitItem>) -> Self {
        Self {
            package,
            imports,
            items,
        }
    }
}

#[derive(Debug)]
pub struct Import<'a> {
    pub path: Cow<'a, str>,
    pub r#static: bool,
    pub wildcard: bool
 }

#[derive(Debug)]
pub struct PackageDeclaration<'a> {
    pub name: Cow<'a, str>,
    pub modifiers: Vec<Annotation<'a>>,
    pub documentation: Cow<'a, str>
}

impl<'a> PackageDeclaration<'a> {
    pub(crate) fn new(name: Cow<'a, str>, modifiers: Vec<Annotation<'a>>, documentation: Cow<'a, str>) -> Self {
        Self {
            name,
            modifiers,
            documentation,
        }
    }
}

#[derive(Debug)]
pub enum CompilationUnitItem {
    Class(ClassDeclaration),
    Enum(EnumDeclaration),
    Interface(InterfaceDeclaration),
    Annotation(AnnotationDeclaration),
}

#[derive(Debug)]
pub struct ClassDeclaration {
    // attrs = ("type_parameters", "extends", "implements")
    // attrs = ("name", "body", "documentation",, "modifiers", "annotations")
    // fields, methods, constructors
}

#[derive(Debug)]
pub struct EnumDeclaration {
    // attrs = ("implements",)
    // fields, methods
    // attrs = ("name", "body", "documentation",, "modifiers", "annotations")
    // fields, methods, constructors
}

#[derive(Debug)]
pub struct InterfaceDeclaration {
    // attrs = ("type_parameters", "extends",)
    // attrs = ("name", "body", "documentation",, "modifiers", "annotations")
    // fields, methods, constructors
}

#[derive(Debug)]
pub struct AnnotationDeclaration {
    // attrs = ("name", "body", "documentation",, "modifiers", "annotations")
    // fields, methods, constructors
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

#[derive(Debug)]
pub struct Annotation<'a> {
    pub name: Cow<'a, str>,
    // element:
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
    fn name(&self) {
    }
}
