use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(i128, SymI128);

impl_val_bin_ops!(i128, Add, add);
impl_val_bin_ops!(i128, Sub, sub);
impl_val_bin_ops!(i128, Mul, mul);
impl_val_bin_ops!(i128, Div, div);
