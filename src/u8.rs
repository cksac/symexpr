use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(U8, U8Ctx, u8);

std_bin_op!(U8, Add, add, u8);
std_bin_op!(U8, Sub, sub, u8);
std_bin_op!(U8, Mul, mul, u8);
std_bin_op!(U8, Div, div, u8);
