use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(i64, SymI64);

impl_val_bin_ops!(i64, Add, add);
impl_val_bin_ops!(i64, Sub, sub);
impl_val_bin_ops!(i64, Mul, mul);
impl_val_bin_ops!(i64, Div, div);
impl_val_bin_ops!(i64, BitAnd, bitand);
impl_val_bin_ops!(i64, BitOr, bitor);
impl_val_bin_ops!(i64, BitXor, bitxor);
impl_val_bin_ops!(i64, Shl, shl);
impl_val_bin_ops!(i64, Shr, shr);
impl_val_bin_ops!(i64, Rem, rem);