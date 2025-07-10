use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(F64, f64);

std_bin_op!(F64, Add, add, f64);
std_bin_op!(F64, Sub, sub, f64);
std_bin_op!(F64, Mul, mul, f64);
std_bin_op!(F64, Div, div, f64);
