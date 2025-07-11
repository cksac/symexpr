use std::marker::PhantomData;

use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

#[derive(Debug)]
pub struct Mul<C, L, R>
where
    L: SymValue<C>,
    R: SymValue<C>,
{
    lhs: L,
    rhs: R,
    ctx: PhantomData<fn(&C)>,
}

impl<C, L, R> Mul<C, L, R>
where
    L: SymValue<C>,
    R: SymValue<C>,
{
    pub fn new(lhs: L, rhs: R) -> Self {
        Self {
            lhs,
            rhs,
            ctx: PhantomData,
        }
    }
}

impl<C, L, R> Clone for Mul<C, L, R>
where
    L: SymValue<C> + Clone,
    R: SymValue<C> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            ctx: PhantomData,
        }
    }
}

impl<C, L, R, LHS, RHS, OUT> SymValue<C> for Mul<C, L, R>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    L: SymValue<C, Value = LHS>,
    R: SymValue<C, Value = RHS>,
    OUT: Value,
    LHS: std::ops::Mul<RHS, Output = OUT>,
{
    type Value = OUT;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs * rhs)
    }
}

impl<C, E, R, LHS, RHS, OUT> std::ops::Mul<R> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<OUT>,
    R: SymValue<C, Value = RHS> + 'static,
    LHS: Value + std::ops::Mul<RHS, Output = OUT>,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;

    fn mul(self, rhs: R) -> Self::Output {
        Sym::Expr(E::wrap(Mul::new(self, rhs)))
    }
}

impl<C, E, R, LHS, RHS, OUT> std::ops::Mul<R> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<OUT>,
    R: SymValue<C, Value = RHS> + 'static,
    LHS: Value + std::ops::Mul<RHS, Output = OUT>,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;

    fn mul(self, rhs: R) -> Self::Output {
        Sym::Expr(E::wrap(Mul::new(self.clone(), rhs)))
    }
}
