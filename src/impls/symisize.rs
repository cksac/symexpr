use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(isize, SymIsize);

impl_val_bin_ops!(isize, Add, add);
impl_val_bin_ops!(isize, Sub, sub);
impl_val_bin_ops!(isize, Mul, mul);
impl_val_bin_ops!(isize, Div, div);
impl_val_bin_ops!(isize, BitAnd, bitand);
impl_val_bin_ops!(isize, BitOr, bitor);
impl_val_bin_ops!(isize, BitXor, bitxor);
