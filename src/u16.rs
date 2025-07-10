use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(U16, u16);

std_bin_op!(U16, Add, add, u16);
std_bin_op!(U16, Sub, sub, u16);
std_bin_op!(U16, Mul, mul, u16);
std_bin_op!(U16, Div, div, u16);
