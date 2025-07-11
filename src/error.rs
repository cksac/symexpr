use std::any::type_name;

use crate::Symbol;

#[derive(Debug)]
pub enum SymError {
    UndefinedSymbol(Symbol, &'static str),
}

impl SymError {
    pub fn undefined_symbol<T>(symbol: Symbol) -> Self {
        Self::UndefinedSymbol(symbol, type_name::<T>())
    }
}

pub type Result<T> = std::result::Result<T, SymError>;
