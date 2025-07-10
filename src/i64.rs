use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(I64, i64);

std_bin_op!(I64, Add, add, i64);
std_bin_op!(I64, Sub, sub, i64);
std_bin_op!(I64, Mul, mul, i64);
std_bin_op!(I64, Div, div, i64);
