use std::collections::HashMap;
use std::fmt::Debug;

use crate::{Result, SymError, Symbol, Value};

pub trait SymCtx<V>: Debug + 'static {
    fn get(&self, symbol: Symbol) -> Result<V>;
    fn bind(&mut self, symbol: impl AsRef<str>, value: V);
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
