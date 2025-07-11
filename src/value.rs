use std::fmt::Debug;

use crate::{Result, SymCtx};

pub trait Value: Debug + Clone + 'static {}

pub trait SymValue<Context>: Debug {
    type Value: Value;
    fn eval(&self, ctx: &Context) -> Result<Self::Value>;
    fn clone_box(&self) -> Box<dyn SymValue<Context, Value = Self::Value>>;
}

impl<C, V> SymValue<C> for Box<dyn SymValue<C, Value = V>>
where
    C: SymCtx<V>,
    V: Value,
{
    type Value = V;
    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        self.as_ref().eval(ctx)
    }
    fn clone_box(&self) -> Box<dyn SymValue<C, Value = Self::Value>> {
        self.as_ref().clone_box()
    }
}
