use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(u64, SymU64);

impl_val_bin_ops!(u64, Add, add);
impl_val_bin_ops!(u64, Sub, sub);
impl_val_bin_ops!(u64, Mul, mul);
impl_val_bin_ops!(u64, Div, div);
