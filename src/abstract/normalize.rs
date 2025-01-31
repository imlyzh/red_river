use crate::core::{Def, Param, Var};

use super::{Locals, Term};

#[derive(Debug, Clone, Default)]
pub struct Normalizer {
    env: Locals,
}

impl Normalizer {
    pub fn term(&mut self, term: Term) -> Term {
        match term {
            Term::Ref(ref var) => {
                if let Some(r) = self.env.get(&var.id) {
                    self.term(r.clone())
                } else {
                    term
                }
            }
            Term::Universe => term,
            Term::FnType(param, term) => {
                let param = self.param(param);
                let term = self.term(*term);
                Term::FnType(param, Box::new(term))
            }
            Term::Fn(param, term) => {
                let param = self.param(param);
                let term = self.term(*term);
                Term::Fn(param, Box::new(term))
            }
            Term::App(term, term1) => {
                let term = self.term(*term);
                let term1 = self.term(*term1);
                match term {
                    Term::Fn(param, term) => self.subst(&param.var, term1, *term),
                    _ => Term::App(Box::new(term), Box::new(term1)),
                }
            }
        }
    }

    fn param(&mut self, param: Param<Box<Term>>) -> Param<Box<Term>> {
        Param {
            var: param.var,
            r#type: Box::new(self.term(*param.r#type)),
        }
    }

    pub fn subst(&mut self, key: &Var, value: Term, term: Term) -> Term {
        self.env.insert(key.id, value);
        self.term(term)
    }

    pub fn apply(&mut self, f: Term, x: Term) -> Term {
        match f {
            Term::Fn(param, term) => self.subst(&param.var, x, *term),
            _ => Term::App(Box::new(f), Box::new(x)),
        }
    }
}

pub fn def_to_value(def: Def<Box<Term>>) -> Term {
    let mut ret = *def.body;
    for p in def.params.into_iter().rev() {
        ret = Term::Fn(p, Box::new(ret));
    }
    ret
}

pub fn def_to_type(def: Def<Box<Term>>) -> Term {
    let mut ret = *def.ret_type;
    for p in def.params.into_iter().rev() {
        ret = Term::FnType(p, Box::new(ret));
    }
    ret
}
