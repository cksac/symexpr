use std::collections::HashMap;

use crate::{Result, SymCtx, SymError, SymValue, Symbol, Value, std_bin_op};

type ValType = i128;
impl Value for ValType {}

#[derive(Debug)]
pub enum I128<C>
where
    C: SymCtx<ValType>,
{
    Const(ValType),
    Symbol(Symbol),
    Expr(Box<dyn SymValue<C, Value = ValType>>),
}

std_bin_op!(I128, Add, add, ValType);
std_bin_op!(I128, Sub, sub, ValType);
std_bin_op!(I128, Mul, mul, ValType);
std_bin_op!(I128, Div, div, ValType);

#[derive(Debug, Default)]
pub struct I128Ctx {
    bindings: HashMap<Symbol, ValType>,
}

impl<C> SymValue<C> for I128<C>
where
    C: SymCtx<ValType>,
{
    type Value = ValType;
    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        match self {
            I128::Const(v) => Ok(*v),
            I128::Symbol(s) => ctx.get(*s),
            I128::Expr(e) => e.eval(ctx),
        }
    }

    fn cloned(&self) -> Box<dyn SymValue<C, Value = Self::Value>> {
        match self {
            I128::Const(v) => Box::new(*v),
            I128::Symbol(s) => Box::new(I128::Symbol(*s)),
            I128::Expr(e) => e.cloned(),
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

impl<C> I128<C>
where
    C: SymCtx<ValType>,
{
    pub fn symbol(name: impl AsRef<str>) -> Self {
        I128::Symbol(Symbol::new(name))
    }
}

impl I128Ctx {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}

impl SymCtx<ValType> for I128Ctx {
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
