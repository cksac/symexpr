use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(U64, u64);

std_bin_op!(U64, Add, add, u64);
std_bin_op!(U64, Sub, sub, u64);
std_bin_op!(U64, Mul, mul, u64);
std_bin_op!(U64, Div, div, u64);
