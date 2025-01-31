use super::{normalize::Normalizer, Term};

pub fn unify(lhs: &Term, rhs: &Term) -> bool {
    match (lhs, rhs) {
        (Term::Ref(var), Term::Ref(var1)) => var.name == var1.name && var.id == var1.id,
        (Term::Universe, Term::Universe) => true,
        (Term::FnType(lparam, lterm), Term::FnType(rparam, rterm)) => {
            unify(&lparam.r#type, &rparam.r#type)
                && unify(
                    lterm,
                    &Normalizer::default().subst(
                        &rparam.var,
                        Term::Ref(lparam.var.clone()),
                        *rterm.clone(),
                    ),
                )
        }
        (Term::Fn(lparam, lterm), Term::Fn(rparam, rterm)) => unify(
            lterm,
            &Normalizer::default().subst(
                &rparam.var,
                Term::Ref(lparam.var.clone()),
                *rterm.clone(),
            ),
        ),
        (Term::App(lterm, lterm1), Term::App(rterm, rterm1)) => {
            unify(lterm, rterm) && unify(lterm1, rterm1)
        }
        _ => false,
    }
}
