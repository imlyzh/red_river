use super::{normalize::Normalizer, Term};

#[derive(Debug, Default)]
pub struct Unifyer {}

impl Unifyer {
    pub fn unify(&self, lhs: &Term, rhs: &Term) -> bool {
        match (lhs, rhs) {
            (Term::Ref(var), Term::Ref(var1)) => var.name == var1.name && var.id == var1.id,
            (Term::Universe, Term::Universe) => true,
            (Term::FnType(lparam, lterm), Term::FnType(rparam, rterm)) => {
                if !self.unify(&lparam.r#type, &rparam.r#type) {
                    return false;
                }
                self.unify(
                    lterm,
                    &Normalizer::default().subst(
                        &rparam.var,
                        Term::Ref(lparam.var.clone()),
                        *rterm.clone(),
                    ),
                )
            }
            (Term::Fn(lparam, lterm), Term::Fn(rparam, rterm)) => self.unify(
                lterm,
                &Normalizer::default().subst(
                    &rparam.var,
                    Term::Ref(lparam.var.clone()),
                    *rterm.clone(),
                ),
            ),
            (Term::App(lterm, lterm1), Term::App(rterm, rterm1)) => {
                self.unify(lterm, rterm) && self.unify(lterm1, rterm1)
            }
            _ => false,
        }
    }
}
