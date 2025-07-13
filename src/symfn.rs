use crate::{Result, SymCtx, SymValue, Value};

#[derive(Debug)]
pub struct SymFn<S, I, O> {
    input: S,
    func: fn(I) -> O,
}

impl<S, I, O> SymFn<S, I, O> {
    pub fn new(input: S, func: fn(I) -> O) -> Self {
        SymFn { input, func }
    }
}

impl<C, S1, I1, O> SymValue<C> for SymFn<(S1,), I1, O>
where
    S1: SymValue<C, Value = I1>,
    C: SymCtx<I1>,
    I1: Value,
    O: Value,
{
    type Value = O;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let input = self.input.0.eval(ctx)?;
        Ok((self.func)(input))
    }
}

impl<C, S1, S2, I1, I2, O> SymValue<C> for SymFn<(S1, S2), (I1, I2), O>
where
    S1: SymValue<C, Value = I1>,
    S2: SymValue<C, Value = I2>,
    C: SymCtx<I1> + SymCtx<I2>,
    I1: Value,
    I2: Value,
    O: Value,
{
    type Value = O;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let i1 = self.input.0.eval(ctx)?;
        let i2 = self.input.1.eval(ctx)?;
        let input = (i1, i2);
        Ok((self.func)(input))
    }
}

impl<C, S1, S2, S3, I1, I2, I3, O> SymValue<C> for SymFn<(S1, S2, S3), (I1, I2, I3), O>
where
    S1: SymValue<C, Value = I1>,
    S2: SymValue<C, Value = I2>,
    S3: SymValue<C, Value = I3>,
    C: SymCtx<I1> + SymCtx<I2> + SymCtx<I3>,
    I1: Value,
    I2: Value,
    I3: Value,
    O: Value,
{
    type Value = O;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let i1 = self.input.0.eval(ctx)?;
        let i2 = self.input.1.eval(ctx)?;
        let i3 = self.input.2.eval(ctx)?;
        let input = (i1, i2, i3);
        Ok((self.func)(input))
    }
}
