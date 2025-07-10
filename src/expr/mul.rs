use std::{fmt::Debug, marker::PhantomData};

use crate::{Result, SymCtx, SymValue, Value};

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

impl<C, L, R> SymValue<C> for Mul<C, L, R>
where
    C: SymCtx<L::Value> + SymCtx<R::Value>,
    L: SymValue<C>,
    R: SymValue<C>,
    L::Value: std::ops::Mul<R::Value>,
    <L::Value as std::ops::Mul<R::Value>>::Output: Value,
{
    type Value = <L::Value as std::ops::Mul<R::Value>>::Output;
    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs * rhs)
    }

    fn cloned(&self) -> Box<dyn SymValue<C, Value = Self::Value>> {
        Box::new(Mul::new(self.lhs.cloned(), self.rhs.cloned()))
    }
}
