use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

pub trait SymAnd<C, E, R>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
{
    fn and(self, rhs: R) -> Sym<bool, C, E>;
}

#[derive(Debug, Clone)]
pub struct And<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    lhs: Sym<L, C, E>,
    rhs: Sym<R, C, E>,
}

impl<C, E, L, R> And<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    pub fn new(lhs: Sym<L, C, E>, rhs: Sym<R, C, E>) -> Self {
        And { lhs, rhs }
    }
}

impl<C, E> SymValue<C> for And<C, E, bool, bool>
where
    bool: Value,
    C: SymCtx<bool>,
    E: SymExpr<bool>,
{
    type Value = bool;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs && rhs)
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.lhs.display(f)?;
        f.write_str(" && ")?;
        self.rhs.display(f)?;
        f.write_str(")")
    }
}

impl<C, E> SymAnd<C, E, Sym<bool, C, E>> for Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    bool: Value,
{
    fn and(self, rhs: Sym<bool, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(And::new(self, rhs)))
    }
}

impl<C, E> SymAnd<C, E, &Sym<bool, C, E>> for Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    bool: Value,
{
    fn and(self, rhs: &Sym<bool, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(And::new(self, rhs.clone())))
    }
}

impl<C, E> SymAnd<C, E, Sym<bool, C, E>> for &Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    bool: Value,
{
    fn and(self, rhs: Sym<bool, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(And::new(self.clone(), rhs)))
    }
}

impl<C, E> SymAnd<C, E, &Sym<bool, C, E>> for &Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    bool: Value,
{
    fn and(self, rhs: &Sym<bool, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(And::new(self.clone(), rhs.clone())))
    }
}

impl<C, E, RHS> SymAnd<C, E, RHS> for Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    RHS: Value + Into<bool>,
{
    fn and(self, rhs: RHS) -> Sym<bool, C, E> {
        let rhs = Sym::<bool, C, E>::constant(rhs.into());
        Sym::<bool, C, E>::Expr(E::lift(And::new(self, rhs)))
    }
}

impl<C, E, LHS> Sym<LHS, C, E>
where
    LHS: Value,
    C: SymCtx<LHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<bool>,
{
    #[inline(always)]
    pub fn and<RHS>(&self, rhs: RHS) -> Sym<bool, C, E>
    where
        for<'a> &'a Self: SymAnd<C, E, RHS>,
    {
        SymAnd::and(self, rhs)
    }
}
