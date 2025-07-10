use std::collections::HashMap;

use crate::{Result, SymCtx, SymError, SymValue, Symbol, Value, std_bin_op};

type ValType = i16;
impl Value for ValType {}

#[derive(Debug)]
pub enum I16<C>
where
    C: SymCtx<ValType>,
{
    Const(ValType),
    Symbol(Symbol),
    Expr(Box<dyn SymValue<C, Value = ValType>>),
}

std_bin_op!(I16, Add, add, ValType);
std_bin_op!(I16, Sub, sub, ValType);
std_bin_op!(I16, Mul, mul, ValType);
std_bin_op!(I16, Div, div, ValType);

#[derive(Debug, Default)]
pub struct I16Ctx {
    bindings: HashMap<Symbol, ValType>,
}

impl<C> SymValue<C> for I16<C>
where
    C: SymCtx<ValType>,
{
    type Value = ValType;
    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        match self {
            I16::Const(v) => Ok(*v),
            I16::Symbol(s) => ctx.get(*s),
            I16::Expr(e) => e.eval(ctx),
        }
    }

    fn cloned(&self) -> Box<dyn SymValue<C, Value = Self::Value>> {
        match self {
            I16::Const(v) => Box::new(*v),
            I16::Symbol(s) => Box::new(I16::Symbol(*s)),
            I16::Expr(e) => e.cloned(),
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

impl<C> I16<C>
where
    C: SymCtx<ValType>,
{
    pub fn symbol(name: impl AsRef<str>) -> Self {
        I16::Symbol(Symbol::new(name))
    }
}

impl I16Ctx {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}

impl SymCtx<ValType> for I16Ctx {
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
