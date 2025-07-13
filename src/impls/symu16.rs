use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(u16, SymU16);

impl_val_bin_ops!(u16, Add, add);
impl_val_bin_ops!(u16, Sub, sub);
impl_val_bin_ops!(u16, Mul, mul);
impl_val_bin_ops!(u16, Div, div);
impl_val_bin_ops!(u16, BitAnd, bitand);
impl_val_bin_ops!(u16, BitOr, bitor);
impl_val_bin_ops!(u16, BitXor, bitxor);
