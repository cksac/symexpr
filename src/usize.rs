use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(Usize, UsizeCtx, usize);

std_bin_op!(Usize, Add, add, usize);
std_bin_op!(Usize, Sub, sub, usize);
std_bin_op!(Usize, Mul, mul, usize);
std_bin_op!(Usize, Div, div, usize);

#[cfg(test)]
mod tests {
    use crate::SymCtx;

    use super::*;

    #[test]
    fn it_works() {
        let mut ctx = UsizeCtx::new();
        ctx.bind("a", 2);
        let x = Usize::symbol("a");
        let y = Usize::Const(2);
        let z = (2 + x + y) / 3usize;
        println!("{:?}", z);
        let result = z.eval(&ctx).unwrap();
        assert_eq!(result, 2);
    }
}
