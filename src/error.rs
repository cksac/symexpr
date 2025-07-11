use crate::Symbol;

#[derive(Debug)]
pub enum SymError {
    SymbolNotFound(Symbol),
}

pub type Result<T> = std::result::Result<T, SymError>;
