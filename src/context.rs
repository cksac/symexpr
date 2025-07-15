use std::collections::HashMap;

use crate::{Result, SymCtx, SymError, Symbol, Value};

impl<T> SymCtx<T> for HashMap<Symbol, T>
where
    T: Value,
{
    fn get_symbol(&self, symbol: Symbol) -> Result<T> {
        self.get(&symbol)
            .cloned()
            .ok_or(SymError::undefined_symbol::<T>(symbol))
    }

    fn set_symbol(&mut self, symbol: impl AsRef<str>, value: T) {
        let symbol = Symbol::new(symbol);
        self.insert(symbol, value);
    }

    fn del_symbol(&mut self, symbol: impl AsRef<str>) {
        let symbol = Symbol::new(symbol);
        self.remove(&symbol);
    }
}

#[derive(Debug, Default)]
pub struct Context {
    usize_bindings: HashMap<Symbol, usize>,
    u8_bindings: HashMap<Symbol, u8>,
    u16_bindings: HashMap<Symbol, u16>,
    u32_bindings: HashMap<Symbol, u32>,
    u64_bindings: HashMap<Symbol, u64>,
    u128_bindings: HashMap<Symbol, u128>,
    isize_bindings: HashMap<Symbol, isize>,
    i8_bindings: HashMap<Symbol, i8>,
    i16_bindings: HashMap<Symbol, i16>,
    i32_bindings: HashMap<Symbol, i32>,
    i64_bindings: HashMap<Symbol, i64>,
    i128_bindings: HashMap<Symbol, i128>,
    f32_bindings: HashMap<Symbol, f32>,
    f64_bindings: HashMap<Symbol, f64>,
    bool_bindings: HashMap<Symbol, bool>,
    char_bindings: HashMap<Symbol, char>,
}

macro_rules! context_for {
    ($($T:ident => $F:ident;)+) => {
        $(
        impl SymCtx<$T> for Context {
            fn get_symbol(&self, symbol: Symbol) -> Result<$T> {
                self.$F.get_symbol(symbol)
            }

            fn set_symbol(&mut self, symbol: impl AsRef<str>, value: $T) {
                self.$F.set_symbol(symbol, value);
            }

            fn del_symbol(&mut self, symbol: impl AsRef<str>) {
                self.$F.del_symbol(symbol);
            }
        }

        impl SymCtx<Option<$T>> for Context {
            fn get_symbol(&self, symbol: Symbol) -> Result<Option<$T>> {
                Ok(self.$F.get_symbol(symbol).ok())
            }

            fn set_symbol(&mut self, symbol: impl AsRef<str>, value: Option<$T>) {
                match value {
                    Some(v) => self.$F.set_symbol(symbol, v),
                    None => self.$F.del_symbol(symbol),
                }
            }

            fn del_symbol(&mut self, symbol: impl AsRef<str>) {
                self.$F.del_symbol(symbol);
            }
        }
        )+
    };
}

context_for! {
    usize => usize_bindings;
    u8 => u8_bindings;
    u16 => u16_bindings;
    u32 => u32_bindings;
    u64 => u64_bindings;
    u128 => u128_bindings;
    isize => isize_bindings;
    i8 => i8_bindings;
    i16 => i16_bindings;
    i32 => i32_bindings;
    i64 => i64_bindings;
    i128 => i128_bindings;
    f32 => f32_bindings;
    f64 => f64_bindings;
    bool => bool_bindings;
    char => char_bindings;
}
