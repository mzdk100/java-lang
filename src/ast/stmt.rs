//! Statement types.

use crate::{ident::Ident, span::Span};

use super::{Comment, expr::Expr, item::Modifier, ty::Type};

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
            Self::Empty(s) => *s,
            Self::Block(b) => b.span(),
            Self::Labeled(l) => l.span(),
            Self::Expr(e) => e.span(),
            Self::LocalVarDecl(d) => d.span(),
            Self::If(s) => s.span(),
            Self::Assert(s) => s.span(),
            Self::Switch(s) => s.span(),
            Self::While(s) => s.span(),
            Self::DoWhile(s) => s.span(),
            Self::For(s) => s.span(),
            Self::EnhancedFor(s) => s.span(),
            Self::Break(s) => s.span(),
            Self::Continue(s) => s.span(),
            Self::Return(s) => s.span(),
            Self::Throw(s) => s.span(),
            Self::Synchronized(s) => s.span(),
            Self::Try(s) => s.span(),
            Self::Yield(s) => s.span(),
            Self::ClassDecl(d) => d.span(),
        }
    }
}

/// A block: `{ stmt1; stmt2; ... }`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub leading_comments: Vec<Comment>,
    pub brace_span: (Span, Span),
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.brace_span.0, |c| c.span);
        start.join(self.brace_span.1)
    }
}

/// A labeled statement: `label: stmt`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabeledStmt {
    pub leading_comments: Vec<Comment>,
    pub label: Ident,
    pub colon_span: Span,
    pub stmt: Box<Stmt>,
}

impl LabeledStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.label.span(), |c| c.span);
        start.join(self.stmt.span())
    }
}

/// An expression statement: `expr;`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub leading_comments: Vec<Comment>,
    pub expr: Expr,
    pub semi_span: Span,
}

impl ExprStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.expr.span(), |c| c.span);
        start.join(self.semi_span)
    }
}

/// A local variable declaration statement: `int x = 5, y = 10;`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalVarDeclStmt {
    pub leading_comments: Vec<Comment>,
    pub modifiers: Vec<Modifier>,
    pub ty: LocalVarType,
    pub declarators: Vec<VariableDeclarator>,
    pub semi_span: Span,
}

impl LocalVarDeclStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.semi_span, |c| c.span);
        start.join(self.semi_span)
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
    pub leading_comments: Vec<Comment>,
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
        let start = self
            .leading_comments
            .first()
            .map_or(self.if_span, |c| c.span);
        start.join(end)
    }
}

/// An assert statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssertStmt {
    pub leading_comments: Vec<Comment>,
    pub assert_span: Span,
    pub cond: Expr,
    pub detail: Option<(Span, Expr)>,
    pub semi_span: Span,
}

impl AssertStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.assert_span, |c| c.span);
        start.join(self.semi_span)
    }
}

/// A switch statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchStmt {
    pub leading_comments: Vec<Comment>,
    pub switch_span: Span,
    pub paren_span: (Span, Span),
    pub selector: Expr,
    pub brace_span: (Span, Span),
    pub cases: Vec<SwitchCaseGroup>,
}

impl SwitchStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.switch_span, |c| c.span);
        start.join(self.brace_span.1)
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
    pub leading_comments: Vec<Comment>,
    pub while_span: Span,
    pub paren_span: (Span, Span),
    pub cond: Expr,
    pub body: Box<Stmt>,
}

impl WhileStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.while_span, |c| c.span);
        start.join(self.body.span())
    }
}

/// A do-while statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoWhileStmt {
    pub leading_comments: Vec<Comment>,
    pub do_span: Span,
    pub body: Box<Stmt>,
    pub while_span: Span,
    pub paren_span: (Span, Span),
    pub cond: Expr,
    pub semi_span: Span,
}

impl DoWhileStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.do_span, |c| c.span);
        start.join(self.semi_span)
    }
}

/// A for statement: `for (init; cond; update) body`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmt {
    pub leading_comments: Vec<Comment>,
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
        let start = self
            .leading_comments
            .first()
            .map_or(self.for_span, |c| c.span);
        start.join(self.body.span())
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
    pub leading_comments: Vec<Comment>,
    pub for_span: Span,
    pub paren_span: (Span, Span),
    pub var_decl: LocalVarDeclStmt,
    pub colon_span: Span,
    pub iterable: Expr,
    pub body: Box<Stmt>,
}

impl EnhancedForStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.for_span, |c| c.span);
        start.join(self.body.span())
    }
}

/// A break or continue statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JumpStmt {
    pub leading_comments: Vec<Comment>,
    pub keyword_span: Span,
    pub label: Option<Ident>,
    pub semi_span: Span,
}

impl JumpStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.keyword_span, |c| c.span);
        start.join(self.semi_span)
    }
}

/// A return statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnStmt {
    pub leading_comments: Vec<Comment>,
    pub return_span: Span,
    pub value: Option<Expr>,
    pub semi_span: Span,
}

impl ReturnStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.return_span, |c| c.span);
        start.join(self.semi_span)
    }
}

/// A throw statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrowStmt {
    pub leading_comments: Vec<Comment>,
    pub throw_span: Span,
    pub expr: Expr,
    pub semi_span: Span,
}

impl ThrowStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.throw_span, |c| c.span);
        start.join(self.semi_span)
    }
}

/// A synchronized statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SynchronizedStmt {
    pub leading_comments: Vec<Comment>,
    pub synchronized_span: Span,
    pub paren_span: (Span, Span),
    pub lock: Expr,
    pub body: Block,
}

impl SynchronizedStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.synchronized_span, |c| c.span);
        start.join(self.body.span())
    }
}

/// A try statement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TryStmt {
    /// `try { ... } catches [finally]`
    Basic {
        leading_comments: Vec<Comment>,
        try_span: Span,
        block: Block,
        catches: Vec<CatchClause>,
        finally_block: Option<(Span, Block)>,
    },
    /// `try (resources) { ... } catches [finally]`
    TryWithResources {
        leading_comments: Vec<Comment>,
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
            Self::Basic {
                leading_comments,
                try_span,
                block,
                finally_block,
                ..
            } => {
                let end = match finally_block {
                    Some((_, b)) => b.brace_span.1,
                    None => block.brace_span.1,
                };
                let start = leading_comments.first().map_or(*try_span, |c| c.span);
                start.join(end)
            }
            Self::TryWithResources {
                leading_comments,
                try_span,
                block,
                finally_block,
                ..
            } => {
                let end = match finally_block {
                    Some((_, b)) => b.brace_span.1,
                    None => block.brace_span.1,
                };
                let start = leading_comments.first().map_or(*try_span, |c| c.span);
                start.join(end)
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
    pub leading_comments: Vec<Comment>,
    pub yield_span: Span,
    pub value: Expr,
    pub semi_span: Span,
}

impl YieldStmt {
    pub fn span(&self) -> Span {
        let start = self
            .leading_comments
            .first()
            .map_or(self.yield_span, |c| c.span);
        start.join(self.semi_span)
    }
}
