use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

pub trait SymGt<C, E, R>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
{
    fn gt(self, rhs: R) -> Sym<bool, C, E>;
}

#[derive(Debug, Clone)]
pub struct Gt<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    lhs: Sym<L, C, E>,
    rhs: Sym<R, C, E>,
}

impl<C, E, L, R> Gt<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    pub fn new(lhs: Sym<L, C, E>, rhs: Sym<R, C, E>) -> Self {
        Gt { lhs, rhs }
    }
}

impl<C, E, L, R> SymValue<C> for Gt<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
    L: std::cmp::PartialOrd<R>,
{
    type Value = bool;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs > rhs)
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.lhs.display(f)?;
        f.write_str(" > ")?;
        self.rhs.display(f)?;
        f.write_str(")")
    }
}

impl<C, E, LHS, RHS> SymGt<C, E, Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn gt(self, rhs: Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Gt::new(self, rhs)))
    }
}

impl<C, E, LHS, RHS> SymGt<C, E, &Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn gt(self, rhs: &Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Gt::new(self, rhs.clone())))
    }
}

impl<C, E, LHS, RHS> SymGt<C, E, Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn gt(self, rhs: Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Gt::new(self.clone(), rhs)))
    }
}

impl<C, E, LHS, RHS> SymGt<C, E, &Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn gt(self, rhs: &Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Gt::new(self.clone(), rhs.clone())))
    }
}

impl<C, E, RHS> SymGt<C, E, RHS> for Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    RHS: Value + Into<bool>,
{
    fn gt(self, rhs: RHS) -> Sym<bool, C, E> {
        let rhs = Sym::<bool, C, E>::constant(rhs.into());
        Sym::<bool, C, E>::Expr(E::lift(Gt::new(self, rhs)))
    }
}

impl<C, E, LHS> Sym<LHS, C, E>
where
    LHS: Value,
    C: SymCtx<LHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<bool>,
{
    #[inline(always)]
    pub fn gt<RHS>(&self, rhs: RHS) -> Sym<bool, C, E>
    where
        for<'a> &'a Self: SymGt<C, E, RHS>,
    {
        SymGt::gt(self, rhs)
    }
}
