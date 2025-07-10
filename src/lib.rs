use std::fmt::Debug;

pub type Symbol = symbol_table::GlobalSymbol;

mod u8;
pub use u8::*;

mod u16;
pub use u16::*;

mod u32;
pub use u32::*;

mod u64;
pub use u64::*;

mod u128;
pub use u128::*;

mod usize;
pub use usize::*;

mod i8;
pub use i8::*;

mod i16;
pub use i16::*;

mod i32;
pub use i32::*;

mod i64;
pub use i64::*;

mod i128;
pub use i128::*;

mod isize;
pub use isize::*;

mod f32;
pub use f32::*;

mod f64;
pub use f64::*;

mod char;
pub use char::*;

mod bool;
pub use bool::*;

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
