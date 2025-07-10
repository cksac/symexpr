use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(I16, I16Ctx, i16);

std_bin_op!(I16, Add, add, i16);
std_bin_op!(I16, Sub, sub, i16);
std_bin_op!(I16, Mul, mul, i16);
std_bin_op!(I16, Div, div, i16);
