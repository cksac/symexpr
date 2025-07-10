use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(I128, i128);

std_bin_op!(I128, Add, add, i128);
std_bin_op!(I128, Sub, sub, i128);
std_bin_op!(I128, Mul, mul, i128);
std_bin_op!(I128, Div, div, i128);
