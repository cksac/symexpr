use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(u32, SymU32);

impl_val_bin_ops!(u32, Add, add);
impl_val_bin_ops!(u32, Sub, sub);
impl_val_bin_ops!(u32, Mul, mul);
impl_val_bin_ops!(u32, Div, div);
