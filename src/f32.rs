use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(F32, f32);

std_bin_op!(F32, Add, add, f32);
std_bin_op!(F32, Sub, sub, f32);
std_bin_op!(F32, Mul, mul, f32);
std_bin_op!(F32, Div, div, f32);
