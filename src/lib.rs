pub type Symbol = symbol_table::GlobalSymbol;

mod error;
pub use error::{Result, SymError};

mod sym;
pub use sym::*;

pub mod ops;

// mod macros;

// mod u8;
// pub use u8::*;

// mod u16;
// pub use u16::*;

// mod u32;
// pub use u32::*;

// mod u64;
// pub use u64::*;

// mod u128;
// pub use u128::*;

mod usize;
pub use usize::*;

// mod i8;
// pub use i8::*;

// mod i16;
// pub use i16::*;

// mod i32;
// pub use i32::*;

// mod i64;
// pub use i64::*;

// mod i128;
// pub use i128::*;

// mod isize;
// pub use isize::*;

// mod f32;
// pub use f32::*;

// mod f64;
// pub use f64::*;

// mod char;
// pub use char::*;

// mod bool;
// pub use bool::*;
