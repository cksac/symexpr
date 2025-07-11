use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(i64, SymI64);

impl_val_bin_ops!(i64, Add, add);
impl_val_bin_ops!(i64, Sub, sub);
impl_val_bin_ops!(i64, Mul, mul);
impl_val_bin_ops!(i64, Div, div);
