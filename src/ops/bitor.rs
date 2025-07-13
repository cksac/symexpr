use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

#[derive(Debug, Clone)]
pub struct BitOr<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    lhs: Sym<L, C, E>,
    rhs: Sym<R, C, E>,
}

impl<C, E, L, R> BitOr<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    pub fn new(lhs: Sym<L, C, E>, rhs: Sym<R, C, E>) -> Self {
        BitOr { lhs, rhs }
    }
}

impl<C, E, L, R, O> SymValue<C> for BitOr<C, E, L, R>
where
    L: Value,
    R: Value,
    O: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
    L: std::ops::BitOr<R, Output = O>,
{
    type Value = O;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs | rhs)
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::BitOr<Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::BitOr<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn bitor(self, rhs: Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(BitOr::new(self, rhs)))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::BitOr<Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::BitOr<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn bitor(self, rhs: Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(BitOr::new(self.clone(), rhs)))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::BitOr<&Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::BitOr<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn bitor(self, rhs: &Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(BitOr::new(self, rhs.clone())))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::BitOr<&Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::BitOr<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn bitor(self, rhs: &Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(BitOr::new(self.clone(), rhs.clone())))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::BitOr<RHS> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::BitOr<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn bitor(self, rhs: RHS) -> Sym<OUT, C, E> {
        let rhs = Sym::<RHS, C, E>::constant(rhs);
        Sym::<OUT, C, E>::Expr(E::lift(BitOr::new(self, rhs)))
    }
}
