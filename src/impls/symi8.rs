use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(i8, SymI8);

impl_val_bin_ops!(i8, Add, add);
impl_val_bin_ops!(i8, Sub, sub);
impl_val_bin_ops!(i8, Mul, mul);
impl_val_bin_ops!(i8, Div, div);
