pub mod normalize;
pub mod unify;

use std::collections::HashMap;

use crate::core::{Def, Param, Var, ID};

#[derive(Debug, Clone)]
pub enum Term {
    Ref(Var),
    Universe,
    FnType(Param<Box<Term>>, Box<Term>),
    Fn(Param<Box<Term>>, Box<Term>),
    App(Box<Term>, Box<Term>),
}

pub type Globals = HashMap<ID, Def<Box<Term>>>;
pub type Locals = HashMap<ID, Term>;
