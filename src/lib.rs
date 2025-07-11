pub type Symbol = symbol_table::GlobalSymbol;

mod error;
pub use error::{Result, SymError};

mod sym;
pub use sym::*;

pub mod ops;

mod macros;

mod impls;