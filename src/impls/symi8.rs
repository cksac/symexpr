use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(i8, SymI8);

impl_val_bin_ops!(i8, Add, add);
impl_val_bin_ops!(i8, Sub, sub);
impl_val_bin_ops!(i8, Mul, mul);
impl_val_bin_ops!(i8, Div, div);
impl_val_bin_ops!(i8, BitAnd, bitand);
impl_val_bin_ops!(i8, BitOr, bitor);
impl_val_bin_ops!(i8, BitXor, bitxor);
impl_val_bin_ops!(i8, Shl, shl);
impl_val_bin_ops!(i8, Shr, shr);
impl_val_bin_ops!(i8, Rem, rem);
