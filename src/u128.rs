use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(U128, u128);

std_bin_op!(U128, Add, add, u128);
std_bin_op!(U128, Sub, sub, u128);
std_bin_op!(U128, Mul, mul, u128);
std_bin_op!(U128, Div, div, u128);
