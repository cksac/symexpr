use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(I8, i8);

std_bin_op!(I8, Add, add, i8);
std_bin_op!(I8, Sub, sub, i8);
std_bin_op!(I8, Mul, mul, i8);
std_bin_op!(I8, Div, div, i8);
