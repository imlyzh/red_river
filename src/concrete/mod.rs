pub mod elaborator;

use crate::core::{Loc, Param, Var};

pub struct Expr {
    pub loc: Loc,
    pub raw_expr: RawExpr,
}

pub enum RawExpr {
    Fn { v: Var, body: Box<Expr> },
    FnType { p: Param<Box<Expr>>, body: Box<Expr> },
    App { f: Box<Expr>, x: Box<Expr> },
    Universe,
    Unresolved(Var),
    Resolved(Var),
}
