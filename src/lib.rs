use std::fmt::Debug;

pub type Symbol = symbol_table::GlobalSymbol;

mod usize;
pub use usize::{Usize, UsizeCtx};

mod i8;
pub use i8::{I8, I8Ctx};

mod i16;
pub use i16::{I16, I16Ctx};

mod i32;
pub use i32::{I32, I32Ctx};

mod i64;
pub use i64::{I64, I64Ctx};

mod i128;
pub use i128::{I128, I128Ctx};

pub mod expr;
pub mod macros;

pub trait Value: Debug + Clone + 'static {}

pub trait SymValue<Context>: Debug {
    type Value: Value;
    fn eval(&self, context: &Context) -> Result<Self::Value>;
    fn cloned(&self) -> Box<dyn SymValue<Context, Value = Self::Value>>;
}

impl<C, V> SymValue<C> for Box<dyn SymValue<C, Value = V>>
where
    C: SymCtx<V>,
    V: Value,
{
    type Value = V;
    fn eval(&self, context: &C) -> Result<Self::Value> {
        self.as_ref().eval(context)
    }
    fn cloned(&self) -> Box<dyn SymValue<C, Value = Self::Value>> {
        self.as_ref().cloned()
    }
}

pub trait SymCtx<V>: Debug + 'static {
    fn get(&self, symbol: Symbol) -> Result<V>;
    fn bind(&mut self, symbol: impl AsRef<str>, value: V);
}

#[derive(Debug)]
pub enum SymError {
    SymbolNotFound(Symbol),
}

pub type Result<T> = std::result::Result<T, SymError>;
