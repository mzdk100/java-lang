mod compilation_unit;
mod documentation_comment;
mod import;
mod module;
mod package;
mod top_level;

use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result as FmtResult},
};
pub use {
    compilation_unit::*, documentation_comment::*, import::*, module::*, package::*, top_level::*,
};

#[derive(Debug)]
pub struct ClassDeclaration<'a> {
    pub name: Cow<'a, str>,
    // attrs = ("type_parameters", "extends", "implements")
    // attrs = ("body", "modifiers", "annotations")
    pub documentation: Option<DocumentationComment<'a>>,
    // fields, methods, constructors
}

impl<'a> Display for ClassDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(ref d) = self.documentation {
            Display::fmt(d, f)?;
        }
        write!(f, "class {}", self.name)
    }
}

#[derive(Debug)]
pub struct EnumDeclaration<'a> {
    pub name: Cow<'a, str>,
    // attrs = ("implements",)
    // fields, methods
    // attrs = ("body", "modifiers", "annotations")
    pub documentation: Option<DocumentationComment<'a>>,
    // fields, methods, constructors
}

impl<'a> Display for EnumDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(ref d) = self.documentation {
            Display::fmt(d, f)?;
        }
        write!(f, "enum {}", self.name)
    }
}

#[derive(Debug)]
pub struct InterfaceDeclaration<'a> {
    pub name: Cow<'a, str>,
    // attrs = ("type_parameters", "extends",)
    // attrs = ("body", "modifiers", "annotations")
    pub documentation: Option<DocumentationComment<'a>>,
    // fields, methods, constructors
}

impl<'a> Display for InterfaceDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(ref d) = self.documentation {
            Display::fmt(d, f)?;
        }
        write!(f, "interface {}", self.name)
    }
}

#[derive(Debug)]
pub struct AnnotationDeclaration<'a> {
    pub name: Cow<'a, str>,
    // attrs = ("body", "modifiers", "annotations")
    pub documentation: Option<DocumentationComment<'a>>,
    // fields, methods, constructors
}

impl<'a> Display for AnnotationDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(ref d) = self.documentation {
            Display::fmt(d, f)?;
        }
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
