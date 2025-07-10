use std::{fmt::Debug, marker::PhantomData};

use crate::{Result, SymCtx, SymValue, Value};

#[derive(Debug)]
pub struct Add<C, L, R>
where
    L: SymValue<C>,
    R: SymValue<C>,
{
    lhs: L,
    rhs: R,
    ctx: PhantomData<fn(&C)>,
}

impl<C, L, R> Add<C, L, R>
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

impl<C, L, R> SymValue<C> for Add<C, L, R>
where
    C: SymCtx<L::Value> + SymCtx<R::Value>,
    L: SymValue<C>,
    R: SymValue<C>,
    L::Value: std::ops::Add<R::Value>,
    <L::Value as std::ops::Add<R::Value>>::Output: Value,
{
    type Value = <L::Value as std::ops::Add<R::Value>>::Output;
    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs + rhs)
    }

    fn cloned(&self) -> Box<dyn SymValue<C, Value = Self::Value>> {
        Box::new(Add::new(self.lhs.cloned(), self.rhs.cloned()))
    }
}
