use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(usize, SymUsize);

impl_val_bin_ops!(usize, Add, add);
impl_val_bin_ops!(usize, Sub, sub);
impl_val_bin_ops!(usize, Mul, mul);
impl_val_bin_ops!(usize, Div, div);
