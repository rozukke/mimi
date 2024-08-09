use crate::span::{Ident, Span};

/// Global level statements
pub struct Item<K = ItemKind> {
    ident: Ident,
    kind: K,
    span: Span,
}

/// Global level statement types
pub enum ItemKind {
    Fn(Box<Fn>),
}

/// Node representing a single function
pub struct Fn {
    pub body: Option<Box<Stmt>>,
}

/// Series of nested expressions
pub struct Stmt {
    kind: StmtKind,
    span: Span,
}

/// Statement type listing
#[derive(Clone, Debug)]
pub enum StmtKind {
    Expr(Box<Expr>),
}

/// Evaluates to a scalar
#[derive(Clone, Debug)]
pub struct Expr {
    kind: ExprKind,
    span: Span,
}

/// Expression type listing
#[derive(Clone, Debug)]
pub enum ExprKind {
    Return(Option<Box<u32>>),
}
