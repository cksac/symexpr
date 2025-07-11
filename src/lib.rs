pub type Symbol = symbol_table::GlobalSymbol;

mod error;
pub use error::{Result, SymError};

mod sym;
pub use sym::*;

mod context;
pub use context::*;

pub mod ops;

mod macros;

mod impls;
