use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(U32, U32Ctx, u32);

std_bin_op!(U32, Add, add, u32);
std_bin_op!(U32, Sub, sub, u32);
std_bin_op!(U32, Mul, mul, u32);
std_bin_op!(U32, Div, div, u32);
