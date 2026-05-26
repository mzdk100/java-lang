//! Expression types.

use crate::{ident::Ident, span::Span};

use super::{
    attribute::Annotation,
    lit::Lit,
    op::{AssignOpToken, BinOpToken, UnaryOp},
    pat::Pattern,
    path::{Path, TypeArguments},
    stmt::Block,
    ty::Type,
};

/// A Java expression.
///
/// Expressions are represented in a mostly-desugared form, following the JLS
/// grammar hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    /// A literal value.
    Literal(Lit),
    /// An identifier expression.
    Ident(Ident),
    /// `this`
    This(Span),
    /// `Super`
    Super(Span),
    /// Parenthesized expression: `(expr)`
    Paren {
        paren_span: (Span, Span),
        expr: Box<Expr>,
    },
    /// Class literal: `String.class`, `int[].class`
    ClassLit {
        type_expr: Box<Type>,
        dot_span: Span,
        class_span: Span,
    },
    /// Field access: `expr.field`
    FieldAccess(FieldAccessExpr),
    /// Method invocation: `method(args)`, `obj.method(args)`
    MethodCall(MethodCallExpr),
    /// Array access: `array[index]`
    ArrayAccess(ArrayAccessExpr),
    /// Method reference: `String::valueOf`, `System.out::println`
    MethodRef(MethodRefExpr),
    /// Array creation: `new int[]{1, 2, 3}`, `new String[10]`
    ArrayNew(ArrayNewExpr),
    /// Type cast: `(int) expr`
    Cast(CastExpr),
    /// Binary operation: `a + b`
    Binary(BinaryExpr),
    /// Unary operation: `-x`, `!flag`
    Unary(UnaryExpr),
    /// instanceof: `obj instanceof String`, `obj instanceof String s`
    Instanceof(InstanceofExpr),
    /// Assignment: `x = 5`, `x += 1`
    Assign(AssignExpr),
    /// Ternary conditional: `cond ? a : b`
    Conditional(ConditionalExpr),
    /// Lambda expression: `x -> x + 1`, `(x, y) -> x + y`
    Lambda(LambdaExpr),
    /// Switch expression (Java 14+): `switch (x) { case 1 -> "one"; ... }`
    Switch(SwitchExpr),
    /// Class instance creation (anonymous class): `new Foo() { ... }`
    NewClass(NewClassExpr),
    /// Array initializer: `{1, 2, 3}`
    ArrayInit(ArrayInitExpr),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(lit) => lit.span(),
            Expr::Ident(ident) => ident.span(),
            Expr::This(s) => *s,
            Expr::Super(s) => *s,
            Expr::Paren { paren_span, .. } => paren_span.0.join(paren_span.1),
            Expr::ClassLit {
                type_expr,
                class_span,
                ..
            } => type_expr.span().join(*class_span),
            Expr::FieldAccess(e) => e.span(),
            Expr::MethodCall(e) => e.span(),
            Expr::ArrayAccess(e) => e.span(),
            Expr::MethodRef(e) => e.span(),
            Expr::ArrayNew(e) => e.span(),
            Expr::Cast(e) => e.span(),
            Expr::Binary(e) => e.span(),
            Expr::Unary(e) => e.span(),
            Expr::Instanceof(e) => e.span(),
            Expr::Assign(e) => e.span(),
            Expr::Conditional(e) => e.span(),
            Expr::Lambda(e) => e.span,
            Expr::Switch(e) => e.span(),
            Expr::NewClass(e) => e.span(),
            Expr::ArrayInit(e) => e.brace_span.0.join(e.brace_span.1),
        }
    }
}

/// Field access expression: `expr.field`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldAccessExpr {
    pub target: Box<Expr>,
    pub dot_span: Span,
    pub field: Ident,
}

impl FieldAccessExpr {
    pub fn span(&self) -> Span {
        self.target.span().join(self.field.span())
    }
}

/// Method invocation expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodCallExpr {
    /// The method receiver (None for unqualified calls like `method()`).
    pub receiver: Option<Box<Expr>>,
    /// Type arguments for the method call.
    pub type_args: Option<TypeArguments>,
    /// The method name.
    pub method: Ident,
    /// Parenthesized arguments.
    pub paren_span: (Span, Span),
    /// The arguments passed to the method.
    pub args: Vec<Expr>,
}

impl MethodCallExpr {
    pub fn span(&self) -> Span {
        let start = match &self.receiver {
            Some(r) => r.span(),
            None => self.method.span(),
        };
        start.join(self.paren_span.1)
    }
}

/// Array access expression: `array[index]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayAccessExpr {
    pub array: Box<Expr>,
    pub index: Box<Expr>,
    pub bracket_span: (Span, Span),
}

impl ArrayAccessExpr {
    pub fn span(&self) -> Span {
        self.array.span().join(self.bracket_span.1)
    }
}

/// Method reference expression: `String::valueOf`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodRefExpr {
    pub target: MethodRefTarget,
    pub colon_colon_span: Span,
    pub type_args: Option<TypeArguments>,
    pub method_name: Ident,
}

impl MethodRefExpr {
    pub fn span(&self) -> Span {
        let start = self.target.span();
        start.join(self.method_name.span())
    }
}

/// The target of a method reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodRefTarget {
    /// `TypeName::method`
    Type(Path),
    /// `expr::method`
    Expr(Box<Expr>),
    /// `super::method`
    Super(Span),
    /// `TypeName.super::method`
    SuperFromType {
        type_name: Path,
        dot_span: Span,
        super_span: Span,
    },
}

impl MethodRefTarget {
    pub fn span(&self) -> Span {
        match self {
            MethodRefTarget::Type(p) => p.span,
            MethodRefTarget::Expr(e) => e.span(),
            MethodRefTarget::Super(s) => *s,
            MethodRefTarget::SuperFromType {
                type_name,
                super_span,
                ..
            } => type_name.span.join(*super_span),
        }
    }
}

/// Class instance creation expression with optional body (anonymous class).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewClassExpr {
    pub new_span: Span,
    pub type_args: Option<TypeArguments>,
    pub class_type: Path,
    pub paren_span: (Span, Span),
    pub args: Vec<Expr>,
    pub body: Option<super::item::ClassBodyDeclList>,
}

impl NewClassExpr {
    pub fn span(&self) -> Span {
        let end = match &self.body {
            Some(b) => b.brace_span.1,
            None => self.paren_span.1,
        };
        self.new_span.join(end)
    }
}

/// Array creation expression: `new int[10]` or `new int[]{1, 2, 3}`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayNewExpr {
    pub new_span: Span,
    pub elem_type: Type,
    pub dim_exprs: Vec<ArrayDimExpr>,
    pub dims: Vec<super::ty::ArrayDim>,
    pub initializer: Option<ArrayInitExpr>,
}

impl ArrayNewExpr {
    pub fn span(&self) -> Span {
        let end = match &self.initializer {
            Some(init) => init.brace_span.1,
            None => {
                if let Some(dim) = self.dims.last() {
                    dim.bracket_span.1
                } else if let Some(dim_expr) = self.dim_exprs.last() {
                    dim_expr.bracket_span.1
                } else {
                    self.elem_type.span()
                }
            }
        };
        self.new_span.join(end)
    }
}

/// A dimension expression in array creation: `[expr]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayDimExpr {
    pub annotations: Vec<Annotation>,
    pub bracket_span: (Span, Span),
    pub expr: Box<Expr>,
}

/// Array initializer: `{1, 2, 3}`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayInitExpr {
    pub brace_span: (Span, Span),
    pub elements: Vec<Expr>,
    pub trailing_comma: bool,
}

/// Type cast expression: `(int) expr`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CastExpr {
    pub paren_span: (Span, Span),
    pub target_type: Type,
    pub expr: Box<Expr>,
}

impl CastExpr {
    pub fn span(&self) -> Span {
        self.paren_span.0.join(self.expr.span())
    }
}

/// Binary expression: `a + b`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: BinOpToken,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn span(&self) -> Span {
        self.left.span().join(self.right.span())
    }
}

/// Unary expression: `-x`, `!flag`, `++x`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub op_span: Span,
    pub expr: Box<Expr>,
    /// True if operator is postfix (e.g., `x++`).
    pub is_postfix: bool,
}

impl UnaryExpr {
    pub fn span(&self) -> Span {
        if self.is_postfix {
            self.expr.span().join(self.op_span)
        } else {
            self.op_span.join(self.expr.span())
        }
    }
}

/// instanceof expression: `obj instanceof String` or `obj instanceof String s`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstanceofExpr {
    pub expr: Box<Expr>,
    pub instanceof_span: Span,
    pub pattern: InstanceofPattern,
}

impl InstanceofExpr {
    pub fn span(&self) -> Span {
        match &self.pattern {
            InstanceofPattern::Type(t) => self.expr.span().join(t.span()),
            InstanceofPattern::Pattern(p) => self.expr.span().join(p.span()),
        }
    }
}

/// The right-hand side of an instanceof expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstanceofPattern {
    /// Simple type check: `obj instanceof String`
    Type(Type),
    /// Pattern matching: `obj instanceof String s`
    Pattern(Pattern),
}

/// Assignment expression: `x = 5`, `x += 1`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignExpr {
    pub target: AssignTarget,
    pub op: AssignOpToken,
    pub value: Box<Expr>,
}

impl AssignExpr {
    pub fn span(&self) -> Span {
        self.target.span().join(self.value.span())
    }
}

/// The target of an assignment expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignTarget {
    /// Simple variable name: `x`
    Ident(Ident),
    /// Field access: `obj.field`
    FieldAccess(FieldAccessExpr),
    /// Array access: `arr[i]`
    ArrayAccess(ArrayAccessExpr),
}

impl AssignTarget {
    pub fn span(&self) -> Span {
        match self {
            AssignTarget::Ident(i) => i.span(),
            AssignTarget::FieldAccess(f) => f.span(),
            AssignTarget::ArrayAccess(a) => a.span(),
        }
    }
}

/// Ternary conditional expression: `cond ? a : b`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConditionalExpr {
    pub cond: Box<Expr>,
    pub question_span: Span,
    pub then_expr: Box<Expr>,
    pub colon_span: Span,
    pub else_expr: Box<Expr>,
}

impl ConditionalExpr {
    pub fn span(&self) -> Span {
        self.cond.span().join(self.else_expr.span())
    }
}

/// Lambda expression: `x -> x + 1`, `(x, y) -> { return x + y; }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LambdaExpr {
    pub params: LambdaParams,
    pub arrow_span: Span,
    pub body: LambdaBody,
    pub span: Span,
}

/// Lambda parameters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LambdaParams {
    /// `(Type name, Type name, ...)`
    List {
        paren_span: (Span, Span),
        params: Vec<LambdaParam>,
    },
    /// `(name, name, ...)` (inferred types)
    IdentList {
        paren_span: (Span, Span),
        idents: Vec<Ident>,
    },
    /// `name` (single inferred parameter)
    Single(Ident),
}

/// A lambda parameter with explicit type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LambdaParam {
    pub modifiers: Vec<super::item::Modifier>,
    pub ty: Type,
    pub name: Ident,
}

/// Lambda body: either an expression or a block.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LambdaBody {
    Expr(Box<Expr>),
    Block(Block),
}

impl LambdaBody {
    pub fn span(&self) -> Span {
        match self {
            LambdaBody::Expr(e) => e.span(),
            LambdaBody::Block(b) => b.span(),
        }
    }
}

/// Switch expression (Java 14+): `switch (x) { case 1 -> "one"; ... }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchExpr {
    pub switch_span: Span,
    pub paren_span: (Span, Span),
    pub selector: Box<Expr>,
    pub brace_span: (Span, Span),
    pub cases: Vec<SwitchArm>,
}

impl SwitchExpr {
    pub fn span(&self) -> Span {
        self.switch_span.join(self.brace_span.1)
    }
}

/// A single arm of a switch expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwitchArm {
    /// `case pattern -> expr;`
    Expr(SwitchCase, Span, Expr),
    /// `case pattern -> { ... }`
    Block(SwitchCase, Span, Block),
    /// `case pattern -> throw ...;`
    Throw(SwitchCase, Span, Expr),
    /// `case pattern: statements` (colon-style, from switch statements)
    Colon(SwitchCase, Vec<super::stmt::Stmt>),
}

impl SwitchArm {
    pub fn span(&self) -> Span {
        match self {
            SwitchArm::Expr(case, _arrow, expr) => case.span().join(expr.span()),
            SwitchArm::Block(case, _arrow, block) => case.span().join(block.brace_span.1),
            SwitchArm::Throw(case, _arrow, expr) => case.span().join(expr.span()),
            SwitchArm::Colon(case, stmts) => {
                let end = stmts.last().map(|s| s.span()).unwrap_or(case.span());
                case.span().join(end)
            }
        }
    }
}

/// A case label in a switch.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwitchCase {
    /// `case 1`, `case 1, 2, 3`
    CaseValues { case_span: Span, values: Vec<Expr> },
    /// `case null`
    CaseNull { case_span: Span, null_span: Span },
    /// `case null, default`
    CaseNullDefault {
        case_span: Span,
        null_span: Span,
        default_span: Span,
    },
    /// `case String s`, `case String s when s.length() > 0`
    CasePattern {
        case_span: Span,
        pattern: Pattern,
        guard: Box<Option<super::pat::Guard>>,
    },
    /// `default`
    Default { default_span: Span },
}

impl SwitchCase {
    pub fn span(&self) -> Span {
        match self {
            SwitchCase::CaseValues { case_span, values } => {
                let end = values.last().map(|v| v.span()).unwrap_or(*case_span);
                case_span.join(end)
            }
            SwitchCase::CaseNull {
                case_span,
                null_span,
            } => case_span.join(*null_span),
            SwitchCase::CaseNullDefault {
                case_span,
                default_span,
                ..
            } => case_span.join(*default_span),
            SwitchCase::CasePattern {
                case_span,
                pattern,
                guard,
            } => {
                let end = match guard.as_ref() {
                    Some(g) => g.expr.span(),
                    None => pattern.span(),
                };
                case_span.join(end)
            }
            SwitchCase::Default { default_span } => *default_span,
        }
    }
}
