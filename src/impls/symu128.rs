use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(u128, SymU128);

impl_val_bin_ops!(u128, Add, add);
impl_val_bin_ops!(u128, Sub, sub);
impl_val_bin_ops!(u128, Mul, mul);
impl_val_bin_ops!(u128, Div, div);
impl_val_bin_ops!(u128, BitAnd, bitand);
impl_val_bin_ops!(u128, BitOr, bitor);
impl_val_bin_ops!(u128, BitXor, bitxor);
