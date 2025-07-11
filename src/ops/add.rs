use std::marker::PhantomData;

use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

#[derive(Debug)]
pub struct Add<C, L, R>
where
    L: SymValue,
    R: SymValue,
{
    lhs: L,
    rhs: R,
    ctx: PhantomData<fn(&C)>,
}

impl<C, L, R> Clone for Add<C, L, R>
where
    L: SymValue + Clone,
    R: SymValue + Clone,
{
    fn clone(&self) -> Self {
        Self {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            ctx: PhantomData,
        }
    }
}

impl<C, L, R, LHS, RHS, OUT> SymValue for Add<C, L, R>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    L: SymValue<Value = LHS, Context = C>,
    R: SymValue<Value = RHS, Context = C>,
    OUT: Value,
    LHS: std::ops::Add<RHS, Output = OUT>,
{
    type Value = OUT;

    type Context = C;

    fn eval(&self, ctx: &Self::Context) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs + rhs)
    }
}

impl<C, E, R, LHS, RHS, OUT> std::ops::Add<R> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<OUT>,
    R: SymValue<Value = RHS, Context = C> + 'static,
    LHS: Value + std::ops::Add<RHS, Output = OUT>,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;

    fn add(self, rhs: R) -> Self::Output {
        Sym::Expr(E::wrap(Add {
            lhs: self,
            rhs,
            ctx: PhantomData,
        }))
    }
}

impl<C, E, R, LHS, RHS, OUT> std::ops::Add<R> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<OUT>,
    R: SymValue<Value = RHS, Context = C> + 'static,
    LHS: Value + std::ops::Add<RHS, Output = OUT>,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;

    fn add(self, rhs: R) -> Self::Output {
        Sym::Expr(E::wrap(Add {
            lhs: self.clone(),
            rhs,
            ctx: PhantomData,
        }))
    }
}
