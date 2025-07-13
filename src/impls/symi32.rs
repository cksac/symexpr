use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(i32, SymI32);

impl_val_bin_ops!(i32, Add, add);
impl_val_bin_ops!(i32, Sub, sub);
impl_val_bin_ops!(i32, Mul, mul);
impl_val_bin_ops!(i32, Div, div);
impl_val_bin_ops!(i32, BitAnd, bitand);
impl_val_bin_ops!(i32, BitOr, bitor);
impl_val_bin_ops!(i32, BitXor, bitxor);
