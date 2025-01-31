use crate::{
    r#abstract::{normalize::{def_to_type, def_to_value, Normalizer}, unify::unify, Globals, Locals, Term}, core::{Def, Param}
};

use super::{Expr, RawExpr};

pub struct Elaborator {
    global: Globals,
    local: Locals,
}

impl Elaborator {
    pub fn elaborate(&mut self, defs: Vec<Def<Expr>>) -> Vec<Def<Box<Term>>> {
        defs.into_iter().map(|d| self.elaborate_def(d)).collect()
    }

    pub fn elaborate_def(&mut self, def: Def<Expr>) -> Def<Box<Term>> {
        let mut checked = vec![];
        let mut ps = vec![];
        for p in def.params {
            let typ = self.check(p.r#type, Term::Universe);
            ps.push(Param {
                var: p.var.clone(),
                r#type: Box::new(typ.clone()),
            });
            self.local.insert(p.var.id, typ);
            checked.push(p.var.id);
        }
        let ret_type = self.check(def.ret_type, Term::Universe);
        let body = self.check(def.body, ret_type.clone());
        for v in checked {
            self.local.remove(&v);
        }
        let checked_def = Def {
            loc: def.loc,
            name: def.name.clone(),
            params: ps,
            ret_type: Box::new(ret_type),
            body: Box::new(body),
        };
        self.global.insert(def.name.id, checked_def.clone());
        checked_def
    }

    pub fn check(&mut self, expr: Expr, typ: Term) -> Term {
        match expr.raw_expr {
            RawExpr::Fn { v, body } => match Normalizer::default().term(typ.clone()) {
                Term::FnType(param, term) => {
                    let body_type = Normalizer::default().subst(&param.var, Term::Ref(v), *term);
                    Term::Fn(
                        param.clone(),
                        Box::new(self.guarded_check(param, *body, body_type)),
                    )
                }
                _ => panic!("{:?}: expected '{:?}', got function type", expr.loc, typ),
            },
            _ => {
                let expr_loc = expr.loc.clone();
                let (tm, got) = self.infer(expr);
                let got = Normalizer::default().term(got);
                let typ = Normalizer::default().term(typ);
                if unify(&got, &typ) {
                    tm
                } else {
                    panic!("{:?}: expected '{:?}', got '{:?}'", expr_loc, typ, got)
                }
            }
        }
    }

    pub fn guarded_check(&mut self, param: Param<Box<Term>>, body: Expr, body_type: Term) -> Term {
        self.local.insert(param.var.id, *param.r#type);
        let r = self.check(body, body_type);
        self.local.remove(&param.var.id);
        r
    }

    pub fn infer(&mut self, expr: Expr) -> (Term, Term) {
        match expr.raw_expr {
            RawExpr::Resolved(var) => {
                if let Some(r) = self.local.get(&var.id) {
                    return (Term::Ref(var.clone()), r.clone());
                }
                if let Some(def) = self.global.get(&var.id) {
                    let def = def.clone();
                    return (def_to_value(def.clone()), def_to_type(def))
                }
                unreachable!()
            }
            RawExpr::FnType { p, body } => {
                let p_typ = self.check(*p.r#type, Term::Universe);
                let inferred_p = Param {
                    var: p.var,
                    r#type: Box::new(p_typ),
                };
                let b_tm = self.guarded_check(inferred_p.clone(), *body, Term::Universe);
                (Term::FnType(inferred_p, Box::new(b_tm)), Term::Universe)
            }
            RawExpr::App { f, x } => {
                let (f_tm, f_typ) = self.infer(*f);
                match f_typ {
                    Term::FnType(param, term) => {
                        let p_type = *param.r#type.clone();
                        let x_tm = self.guarded_check(param.clone(), *x, p_type);
                        let typ = Normalizer::default().subst(&param.var, x_tm.clone(), *term);
                        let tm = Normalizer::default().apply(f_tm, x_tm);
                        (tm, typ)
                    }
                    typ => {
                        panic!("{:?}: expected function type, got '{:?}'", expr.loc, typ)
                    }
                }
            }
            RawExpr::Universe => (Term::Universe, Term::Universe),
            _ => unreachable!(),
        }
    }
}
