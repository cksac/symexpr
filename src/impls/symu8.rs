use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(u8, SymU8);

impl_val_bin_ops!(u8, Add, add);
impl_val_bin_ops!(u8, Sub, sub);
impl_val_bin_ops!(u8, Mul, mul);
impl_val_bin_ops!(u8, Div, div);
impl_val_bin_ops!(u8, BitAnd, bitand);
impl_val_bin_ops!(u8, BitOr, bitor);
impl_val_bin_ops!(u8, BitXor, bitxor);
impl_val_bin_ops!(u8, Shl, shl);
impl_val_bin_ops!(u8, Shr, shr);
impl_val_bin_ops!(u8, Rem, rem);