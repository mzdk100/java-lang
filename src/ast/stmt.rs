//! Statement types.

use crate::{ident::Ident, span::Span};

use super::{expr::Expr, item::Modifier, ty::Type};

/// A Java statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    /// An empty statement: `;`
    Empty(Span),
    /// A block: `{ ... }`
    Block(Block),
    /// A labeled statement: `label: stmt`
    Labeled(LabeledStmt),
    /// An expression statement: `expr;`
    Expr(ExprStmt),
    /// A local variable declaration: `int x = 5;`
    LocalVarDecl(LocalVarDeclStmt),
    /// An if statement: `if (cond) stmt [else stmt]`
    If(IfStmt),
    /// An assert statement: `assert cond [: detail];`
    Assert(AssertStmt),
    /// A switch statement: `switch (expr) { ... }`
    Switch(SwitchStmt),
    /// A while statement: `while (cond) stmt`
    While(WhileStmt),
    /// A do-while statement: `do stmt while (cond);`
    DoWhile(DoWhileStmt),
    /// A for statement: `for (init; cond; update) stmt`
    For(ForStmt),
    /// An enhanced for statement: `for (Type var : iterable) stmt`
    EnhancedFor(EnhancedForStmt),
    /// A break statement: `break [label];`
    Break(JumpStmt),
    /// A continue statement: `continue [label];`
    Continue(JumpStmt),
    /// A return statement: `return [expr];`
    Return(ReturnStmt),
    /// A throw statement: `throw expr;`
    Throw(ThrowStmt),
    /// A synchronized statement: `synchronized (lock) { ... }`
    Synchronized(SynchronizedStmt),
    /// A try statement: `try { ... } catch (...) { ... } [finally { ... }]`
    Try(TryStmt),
    /// A yield statement: `yield expr;` (in switch expressions)
    Yield(YieldStmt),
    /// A local class or interface declaration.
    ClassDecl(Box<super::item::TypeDecl>),
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Empty(s) => *s,
            Stmt::Block(b) => b.span(),
            Stmt::Labeled(l) => l.span(),
            Stmt::Expr(e) => e.span(),
            Stmt::LocalVarDecl(d) => d.span(),
            Stmt::If(s) => s.span(),
            Stmt::Assert(s) => s.span(),
            Stmt::Switch(s) => s.span(),
            Stmt::While(s) => s.span(),
            Stmt::DoWhile(s) => s.span(),
            Stmt::For(s) => s.span(),
            Stmt::EnhancedFor(s) => s.span(),
            Stmt::Break(s) => s.span(),
            Stmt::Continue(s) => s.span(),
            Stmt::Return(s) => s.span(),
            Stmt::Throw(s) => s.span(),
            Stmt::Synchronized(s) => s.span(),
            Stmt::Try(s) => s.span(),
            Stmt::Yield(s) => s.span(),
            Stmt::ClassDecl(d) => d.span(),
        }
    }
}

/// A block: `{ stmt1; stmt2; ... }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub brace_span: (Span, Span),
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn span(&self) -> Span {
        self.brace_span.0.join(self.brace_span.1)
    }
}

/// A labeled statement: `label: stmt`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabeledStmt {
    pub label: Ident,
    pub colon_span: Span,
    pub stmt: Box<Stmt>,
}

impl LabeledStmt {
    pub fn span(&self) -> Span {
        self.label.span().join(self.stmt.span())
    }
}

/// An expression statement: `expr;`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub expr: Expr,
    pub semi_span: Span,
}

impl ExprStmt {
    pub fn span(&self) -> Span {
        self.expr.span().join(self.semi_span)
    }
}

/// A local variable declaration statement: `int x = 5, y = 10;`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalVarDeclStmt {
    pub modifiers: Vec<Modifier>,
    pub ty: LocalVarType,
    pub declarators: Vec<VariableDeclarator>,
    pub semi_span: Span,
}

impl LocalVarDeclStmt {
    pub fn span(&self) -> Span {
        self.semi_span
    }
}

/// The type of a local variable declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocalVarType {
    /// Explicit type: `int`, `String`, etc.
    Type(Type),
    /// `var` (type inference, Java 10+)
    Var(Span),
}

/// A variable declarator: `name [= initializer]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableDeclarator {
    pub name: Option<Ident>, // None for unnamed variables (Java 21+)
    pub dims: Vec<super::ty::ArrayDim>,
    pub initializer: Option<Expr>,
}

/// An if statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStmt {
    pub if_span: Span,
    pub paren_span: (Span, Span),
    pub cond: Expr,
    pub then_stmt: Box<Stmt>,
    pub else_clause: Option<(Span, Box<Stmt>)>,
}

impl IfStmt {
    pub fn span(&self) -> Span {
        let end = match &self.else_clause {
            Some((_, stmt)) => stmt.span(),
            None => self.then_stmt.span(),
        };
        self.if_span.join(end)
    }
}

/// An assert statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssertStmt {
    pub assert_span: Span,
    pub cond: Expr,
    pub detail: Option<(Span, Expr)>,
    pub semi_span: Span,
}

impl AssertStmt {
    pub fn span(&self) -> Span {
        self.assert_span.join(self.semi_span)
    }
}

/// A switch statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchStmt {
    pub switch_span: Span,
    pub paren_span: (Span, Span),
    pub selector: Expr,
    pub brace_span: (Span, Span),
    pub cases: Vec<SwitchCaseGroup>,
}

impl SwitchStmt {
    pub fn span(&self) -> Span {
        self.switch_span.join(self.brace_span.1)
    }
}

/// A group of case labels and statements in a switch statement (colon-style).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchCaseGroup {
    pub labels: Vec<super::expr::SwitchCase>,
    pub colon_span: Span,
    pub stmts: Vec<Stmt>,
}

/// A while statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileStmt {
    pub while_span: Span,
    pub paren_span: (Span, Span),
    pub cond: Expr,
    pub body: Box<Stmt>,
}

impl WhileStmt {
    pub fn span(&self) -> Span {
        self.while_span.join(self.body.span())
    }
}

/// A do-while statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoWhileStmt {
    pub do_span: Span,
    pub body: Box<Stmt>,
    pub while_span: Span,
    pub paren_span: (Span, Span),
    pub cond: Expr,
    pub semi_span: Span,
}

impl DoWhileStmt {
    pub fn span(&self) -> Span {
        self.do_span.join(self.semi_span)
    }
}

/// A for statement: `for (init; cond; update) body`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmt {
    pub for_span: Span,
    pub paren_span: (Span, Span),
    pub init: ForInit,
    pub cond: Option<Expr>,
    pub semi2_span: Option<Span>,
    pub update: Vec<Expr>,
    pub body: Box<Stmt>,
}

impl ForStmt {
    pub fn span(&self) -> Span {
        self.for_span.join(self.body.span())
    }
}

/// The initialization part of a for statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForInit {
    /// A list of expression statements.
    Exprs(Vec<Expr>),
    /// A local variable declaration.
    LocalVarDecl(LocalVarDeclStmt),
}

/// An enhanced for statement: `for (Type var : iterable) body`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnhancedForStmt {
    pub for_span: Span,
    pub paren_span: (Span, Span),
    pub var_decl: LocalVarDeclStmt,
    pub colon_span: Span,
    pub iterable: Expr,
    pub body: Box<Stmt>,
}

impl EnhancedForStmt {
    pub fn span(&self) -> Span {
        self.for_span.join(self.body.span())
    }
}

/// A break or continue statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JumpStmt {
    pub keyword_span: Span,
    pub label: Option<Ident>,
    pub semi_span: Span,
}

impl JumpStmt {
    pub fn span(&self) -> Span {
        self.keyword_span.join(self.semi_span)
    }
}

/// A return statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnStmt {
    pub return_span: Span,
    pub value: Option<Expr>,
    pub semi_span: Span,
}

impl ReturnStmt {
    pub fn span(&self) -> Span {
        self.return_span.join(self.semi_span)
    }
}

/// A throw statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrowStmt {
    pub throw_span: Span,
    pub expr: Expr,
    pub semi_span: Span,
}

impl ThrowStmt {
    pub fn span(&self) -> Span {
        self.throw_span.join(self.semi_span)
    }
}

/// A synchronized statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SynchronizedStmt {
    pub synchronized_span: Span,
    pub paren_span: (Span, Span),
    pub lock: Expr,
    pub body: Block,
}

impl SynchronizedStmt {
    pub fn span(&self) -> Span {
        self.synchronized_span.join(self.body.span())
    }
}

/// A try statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TryStmt {
    /// `try { ... } catches [finally]`
    Basic {
        try_span: Span,
        block: Block,
        catches: Vec<CatchClause>,
        finally_block: Option<(Span, Block)>,
    },
    /// `try (resources) { ... } catches [finally]`
    TryWithResources {
        try_span: Span,
        paren_span: (Span, Span),
        resources: Vec<TryResource>,
        block: Block,
        catches: Vec<CatchClause>,
        finally_block: Option<(Span, Block)>,
    },
}

impl TryStmt {
    pub fn span(&self) -> Span {
        match self {
            TryStmt::Basic {
                try_span,
                block,
                finally_block,
                ..
            } => {
                let end = match finally_block {
                    Some((_, b)) => b.brace_span.1,
                    None => block.brace_span.1,
                };
                try_span.join(end)
            }
            TryStmt::TryWithResources {
                try_span,
                block,
                finally_block,
                ..
            } => {
                let end = match finally_block {
                    Some((_, b)) => b.brace_span.1,
                    None => block.brace_span.1,
                };
                try_span.join(end)
            }
        }
    }
}

/// A catch clause.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatchClause {
    pub catch_span: Span,
    pub paren_span: (Span, Span),
    pub param: CatchParam,
    pub block: Block,
}

/// The parameter of a catch clause.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatchParam {
    pub modifiers: Vec<Modifier>,
    pub ty: CatchType,
    pub name: Ident,
}

/// The type in a catch parameter (can be a union type with `|`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatchType {
    pub types: Vec<Type>,
}

/// A resource in a try-with-resources statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TryResource {
    /// A variable declaration: `BufferedReader br = new BufferedReader(...)`
    Decl(LocalVarDeclStmt),
    /// A final variable reference: `br` (Java 9+)
    VarRef(Ident),
}

/// A yield statement (in switch expressions).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct YieldStmt {
    pub yield_span: Span,
    pub value: Expr,
    pub semi_span: Span,
}

impl YieldStmt {
    pub fn span(&self) -> Span {
        self.yield_span.join(self.semi_span)
    }
}
