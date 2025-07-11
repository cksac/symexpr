use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(f64, SymF64);

impl_val_bin_ops!(f64, Add, add);
impl_val_bin_ops!(f64, Sub, sub);
impl_val_bin_ops!(f64, Mul, mul);
impl_val_bin_ops!(f64, Div, div);
