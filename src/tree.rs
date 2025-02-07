pub trait Node {}

pub trait CompilationUnit: Node {
    // attrs = ("package", "imports", "types")
}

pub trait Import: Node {
    // attrs = ("path", "static", "wildcard")
}

pub trait Documented: Node {
    // attrs = ("documentation",)
}

pub trait Declaration: Node {
    // attrs = ("modifiers", "annotations")
}

pub trait TypeDeclaration: Declaration + Documented {
    // attrs = ("name", "body")
    // fields, methods, constructors
}

pub trait PackageDeclaration: Declaration + Documented {
    // attrs = ("name",)
}

pub trait ClassDeclaration: TypeDeclaration {
    // attrs = ("type_parameters", "extends", "implements")
}

pub trait EnumDeclaration: TypeDeclaration {
    // attrs = ("implements",)
    // fields, methods
}

pub trait InterfaceDeclaration: TypeDeclaration {
    // attrs = ("type_parameters", "extends",)
}

pub trait AnnotationDeclaration: TypeDeclaration {}

pub trait Type: Node {
    // attrs = ("name", "dimensions",)
}

pub trait BasicType: Type {}

pub trait ReferenceType: Type {
    // attrs = ("arguments", "sub_type")
}

pub trait TypeArgument: Node {
    // attrs = ("type", "pattern_type")
}

pub trait TypeParameter: Node {
    // attrs = ("name", "extends")
}

pub trait Annotation: Node {
    // attrs = ("name", "element")
}

pub trait ElementValuePair: Node {
    // attrs = ("name", "value")
}

pub trait ElementArrayValue: Node {
    // attrs = ("values",)
}

pub trait Member: Documented {}

pub trait MethodDeclaration: Member + Declaration {
    // attrs = ("type_parameters", "return_type", "name", "parameters", "throws", "body")
}

pub trait FieldDeclaration: Member + Declaration {
    // attrs = ("type", "declarators")
}

pub trait ConstructorDeclaration: Declaration + Documented {
    // attrs = ("type_parameters", "name", "parameters", "throws", "body")
}

pub trait ConstantDeclaration: FieldDeclaration {}

pub trait ArrayInitializer: Node {
    // attrs = ("initializers",)
}

pub trait VariableDeclaration: Declaration {
    // attrs = ("type", "declarators")
}

pub trait LocalVariableDeclaration: VariableDeclaration {}

pub trait VariableDeclarator: Node {
    // attrs = ("name", "dimensions", "initializer")
}

pub trait FormalParameter: Declaration {
    // attrs = ("type", "name", "varargs")
}

pub trait InferredFormalParameter: Node {
    // attrs = ('name',)
}

pub trait Statement: Node {
    // attrs = ("label",)
}

pub trait IfStatement: Statement {
    // attrs = ("condition", "then_statement", "else_statement")
}

pub trait WhileStatement: Statement {
    // attrs = ("condition", "body")
}

pub trait DoStatement: Statement {
    // attrs = ("condition", "body")
}

pub trait ForStatement: Statement {
    // attrs = ("control", "body")
}

pub trait AssertStatement: Statement {
    // attrs = ("condition", "value")
}

pub trait BreakStatement: Statement {
    // attrs = ("goto",)
}

pub trait ContinueStatement: Statement {
    // attrs = ("goto",)
}

pub trait ReturnStatement: Statement {
    // attrs = ("expression",)
}

pub trait ThrowStatement: Statement {
    // attrs = ("expression",)
}

pub trait SynchronizedStatement: Statement {
    // attrs = ("lock", "block")
}

pub trait TryStatement: Statement {
    // attrs = ("resources", "block", "catches", "finally_block")
}

pub trait SwitchStatement: Statement {
    // attrs = ("expression", "cases")
}

pub trait BlockStatement: Statement {
    // attrs = ("statements",)
}

pub trait StatementExpression: Statement {
    // attrs = ("expression",)
}

pub trait TryResource: Declaration {
    // attrs = ("type", "name", "value")
}

pub trait CatchClause: Statement {
    // attrs = ("parameter", "block")
}

pub trait CatchClauseParameter: Declaration {
    // attrs = ("types", "name")
}

pub trait SwitchStatementCase: Node {
    // attrs = ("case", "statements")
}

pub trait ForControl: Node {
    // attrs = ("init", "condition", "update")
}

pub trait EnhancedForControl: Node {
    // attrs = ("var", "iterable")
}

pub trait Expression: Node {}

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

pub trait EnumBody: Node {
    // attrs = ("constants", "declarations")
}

pub trait EnumConstantDeclaration: Declaration + Documented {
    // attrs = ("name", "arguments", "body")
}

pub trait AnnotationMethod: Declaration {
    // attrs = ("name", "return_type", "dimensions", "default")
    fn name(&self) {
    }
}
