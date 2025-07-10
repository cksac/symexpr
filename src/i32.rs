use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(I32, i32);

std_bin_op!(I32, Add, add, i32);
std_bin_op!(I32, Sub, sub, i32);
std_bin_op!(I32, Mul, mul, i32);
std_bin_op!(I32, Div, div, i32);
