use std::collections::HashMap;

use crate::{Result, SymCtx, SymError, SymValue, Symbol, Value};

#[derive(Debug)]
pub enum Sym<T, C>
where
    T: Value,
    C: SymCtx<T>,
{
    Const(T),
    Symbol(Symbol),
    Expr(Box<dyn crate::SymValue<C, Value = T>>),
}

impl<C, T> SymValue<C> for Sym<T, C>
where
    T: Value,
    C: SymCtx<T>,
{
    type Value = T;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        match self {
            Self::Const(v) => Ok(v.clone()),
            Self::Symbol(s) => ctx.get(*s),
            Self::Expr(e) => e.eval(ctx),
        }
    }

    fn cloned(&self) -> Box<dyn SymValue<C, Value = Self::Value>> {
        match self {
            Self::Const(v) => Box::new(Self::Const(v.clone())),
            Self::Symbol(s) => Box::new(Self::Symbol(*s)),
            Self::Expr(e) => e.cloned(),
        }
    }
}

impl<T, C> Sym<T, C>
where
    T: Value,
    C: SymCtx<T>,
{
    pub fn symbol(name: impl AsRef<str>) -> Self {
        Self::Symbol(Symbol::new(name))
    }
}

impl<T> SymCtx<T> for HashMap<Symbol, T>
where
    T: Value,
{
    fn get(&self, symbol: Symbol) -> Result<T> {
        self.get(&symbol)
            .cloned()
            .ok_or(SymError::SymbolNotFound(symbol))
    }

    fn bind(&mut self, symbol: impl AsRef<str>, value: T) {
        self.insert(Symbol::new(symbol), value);
    }
}
