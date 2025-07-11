use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(f32, SymF32);

impl_val_bin_ops!(f32, Add, add);
impl_val_bin_ops!(f32, Sub, sub);
impl_val_bin_ops!(f32, Mul, mul);
impl_val_bin_ops!(f32, Div, div);
