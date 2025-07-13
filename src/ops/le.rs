use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

pub trait SymLe<C, E, R>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
{
    fn le(self, rhs: R) -> Sym<bool, C, E>;
}

#[derive(Debug, Clone)]
pub struct Le<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    lhs: Sym<L, C, E>,
    rhs: Sym<R, C, E>,
}

impl<C, E, L, R> Le<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    pub fn new(lhs: Sym<L, C, E>, rhs: Sym<R, C, E>) -> Self {
        Le { lhs, rhs }
    }
}

impl<C, E, L, R> SymValue<C> for Le<C, E, L, R>
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
        Ok(lhs <= rhs)
    }
}

impl<C, E, LHS, RHS> SymLe<C, E, Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn le(self, rhs: Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Le::new(self, rhs)))
    }
}

impl<C, E, LHS, RHS> SymLe<C, E, &Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn le(self, rhs: &Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Le::new(self, rhs.clone())))
    }
}

impl<C, E, LHS, RHS> SymLe<C, E, Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn le(self, rhs: Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Le::new(self.clone(), rhs)))
    }
}

impl<C, E, LHS, RHS> SymLe<C, E, &Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialOrd<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn le(self, rhs: &Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Le::new(self.clone(), rhs.clone())))
    }
}

impl<C, E, RHS> SymLe<C, E, RHS> for Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    RHS: Value + Into<bool>,
{
    fn le(self, rhs: RHS) -> Sym<bool, C, E> {
        let rhs = Sym::<bool, C, E>::constant(rhs.into());
        Sym::<bool, C, E>::Expr(E::lift(Le::new(self, rhs)))
    }
}

impl<C, E, LHS> Sym<LHS, C, E>
where
    LHS: Value,
    C: SymCtx<LHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<bool>,
{
    #[inline(always)]
    pub fn le<RHS>(&self, rhs: RHS) -> Sym<bool, C, E>
    where
        for<'a> &'a Self: SymLe<C, E, RHS>,
    {
        SymLe::le(self, rhs)
    }
}
