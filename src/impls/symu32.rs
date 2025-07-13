use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(u32, SymU32);

impl_val_bin_ops!(u32, Add, add);
impl_val_bin_ops!(u32, Sub, sub);
impl_val_bin_ops!(u32, Mul, mul);
impl_val_bin_ops!(u32, Div, div);
impl_val_bin_ops!(u32, BitAnd, bitand);
impl_val_bin_ops!(u32, BitOr, bitor);
impl_val_bin_ops!(u32, BitXor, bitxor);
impl_val_bin_ops!(u32, Shl, shl);
impl_val_bin_ops!(u32, Shr, shr);
impl_val_bin_ops!(u32, Rem, rem);