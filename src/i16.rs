use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(i16, SymI16);

impl_val_bin_ops!(i16, Add, add);
impl_val_bin_ops!(i16, Sub, sub);
impl_val_bin_ops!(i16, Mul, mul);
impl_val_bin_ops!(i16, Div, div);
