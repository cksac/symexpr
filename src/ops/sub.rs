use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

#[derive(Debug, Clone)]
pub struct Sub<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    lhs: Sym<L, C, E>,
    rhs: Sym<R, C, E>,
}

impl<C, E, L, R> Sub<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    pub fn new(lhs: Sym<L, C, E>, rhs: Sym<R, C, E>) -> Self {
        Sub { lhs, rhs }
    }
}

impl<C, E, L, R, O> SymValue<C> for Sub<C, E, L, R>
where
    L: Value,
    R: Value,
    O: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
    L: std::ops::Sub<R, Output = O>,
{
    type Value = O;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs - rhs)
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Sub<Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Sub<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn sub(self, rhs: Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(Sub::new(self, rhs)))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Sub<Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Sub<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn sub(self, rhs: Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(Sub::new(self.clone(), rhs)))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Sub<&Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Sub<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn sub(self, rhs: &Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(Sub::new(self, rhs.clone())))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Sub<&Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Sub<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn sub(self, rhs: &Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(Sub::new(self.clone(), rhs.clone())))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Sub<RHS> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Sub<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn sub(self, rhs: RHS) -> Sym<OUT, C, E> {
        let rhs = Sym::<RHS, C, E>::constant(rhs);
        Sym::<OUT, C, E>::Expr(E::lift(Sub::new(self, rhs)))
    }
}
