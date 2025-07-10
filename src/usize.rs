use std::collections::HashMap;

use crate::{Result, SymCtx, SymError, SymValue, Symbol, Value, std_bin_op};

type ValType = usize;
impl Value for ValType {}

#[derive(Debug)]
pub enum Usize<C>
where
    C: SymCtx<ValType>,
{
    Const(ValType),
    Symbol(Symbol),
    Expr(Box<dyn SymValue<C, Value = ValType>>),
}

std_bin_op!(Usize, Add, add, ValType);
std_bin_op!(Usize, Sub, sub, ValType);
std_bin_op!(Usize, Mul, mul, ValType);
std_bin_op!(Usize, Div, div, ValType);

#[derive(Debug, Default)]
pub struct UsizeCtx {
    bindings: HashMap<Symbol, ValType>,
}

impl<C> SymValue<C> for Usize<C>
where
    C: SymCtx<ValType>,
{
    type Value = ValType;
    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        match self {
            Usize::Const(v) => Ok(*v),
            Usize::Symbol(s) => ctx.get(*s),
            Usize::Expr(e) => e.eval(ctx),
        }
    }

    fn cloned(&self) -> Box<dyn SymValue<C, Value = Self::Value>> {
        match self {
            Usize::Const(v) => Box::new(*v),
            Usize::Symbol(s) => Box::new(Usize::Symbol(*s)),
            Usize::Expr(e) => e.cloned(),
        }
    }
}

impl<C> SymValue<C> for ValType {
    type Value = ValType;
    fn eval(&self, _ctx: &C) -> Result<Self::Value> {
        Ok(*self)
    }

    fn cloned(&self) -> Box<dyn SymValue<C, Value = Self::Value>> {
        Box::new(*self)
    }
}

impl<C> Usize<C>
where
    C: SymCtx<ValType>,
{
    pub fn symbol(name: impl AsRef<str>) -> Self {
        Usize::Symbol(Symbol::new(name))
    }
}

impl UsizeCtx {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}

impl SymCtx<ValType> for UsizeCtx {
    fn get(&self, symbol: Symbol) -> Result<ValType> {
        self.bindings
            .get(&symbol)
            .copied()
            .ok_or(SymError::SymbolNotFound(symbol))
    }

    fn bind(&mut self, symbol: impl AsRef<str>, value: ValType) {
        self.bindings.insert(Symbol::new(symbol), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut ctx = UsizeCtx::new();
        ctx.bind("a", 2);
        let x = Usize::symbol("a");
        let y = Usize::Const(2);
        let z = (2 + x + y) / 3usize;
        println!("{:?}", z);
        let result = z.eval(&ctx).unwrap();
        assert_eq!(result, 2);
    }
}
