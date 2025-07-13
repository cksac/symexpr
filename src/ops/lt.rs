use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

pub trait SymLt<C, E, R>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
{
    fn lt(self, rhs: R) -> Sym<bool, C, E>;
}

#[derive(Debug, Clone)]
pub struct Lt<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    lhs: Sym<L, C, E>,
    rhs: Sym<R, C, E>,
}

impl<C, E, L, R> Lt<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    pub fn new(lhs: Sym<L, C, E>, rhs: Sym<R, C, E>) -> Self {
        Lt { lhs, rhs }
    }
}

impl<C, E, L, R> SymValue<C> for Lt<C, E, L, R>
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
        Ok(lhs < rhs)
    }
}

impl<C, E, LHS, RHS> SymLt<C, E, Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn lt(self, rhs: Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Lt::new(self, rhs)))
    }
}

impl<C, E, LHS, RHS> SymLt<C, E, &Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn lt(self, rhs: &Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Lt::new(self, rhs.clone())))
    }
}

impl<C, E, LHS, RHS> SymLt<C, E, Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn lt(self, rhs: Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Lt::new(self.clone(), rhs)))
    }
}

impl<C, E, LHS, RHS> SymLt<C, E, &Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn lt(self, rhs: &Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Lt::new(self.clone(), rhs.clone())))
    }
}

impl<C, E, RHS> SymLt<C, E, RHS> for Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    RHS: Value + Into<bool>,
{
    fn lt(self, rhs: RHS) -> Sym<bool, C, E> {
        let rhs = Sym::<bool, C, E>::constant(rhs.into());
        Sym::<bool, C, E>::Expr(E::lift(Lt::new(self, rhs)))
    }
}

impl<C, E, LHS> Sym<LHS, C, E>
where
    LHS: Value,
    C: SymCtx<LHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<bool>,
{
    #[inline(always)]
    pub fn lt<RHS>(&self, rhs: RHS) -> Sym<bool, C, E>
    where
        for<'a> &'a Self: SymLt<C, E, RHS>,
    {
        SymLt::lt(self, rhs)
    }
}
