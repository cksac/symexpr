use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(bool, SymBool);

impl_val_bin_ops!(bool, BitAnd, bitand);
impl_val_bin_ops!(bool, BitOr, bitor);
impl_val_bin_ops!(bool, BitXor, bitxor);
