use crate::{SymValue, define_sym_val, std_bin_op};

define_sym_val!(Isize, IsizeCtx, isize);

std_bin_op!(Isize, Add, add, isize);
std_bin_op!(Isize, Sub, sub, isize);
std_bin_op!(Isize, Mul, mul, isize);
std_bin_op!(Isize, Div, div, isize);

#[cfg(test)]
mod tests {
    use crate::SymCtx;

    use super::*;

    #[test]
    fn it_works() {
        let mut ctx = IsizeCtx::new();
        ctx.bind("a", 2);
        let x = Isize::symbol("a");
        let y = Isize::Const(2);
        let z = (2 + x + y) / 3isize;
        println!("{:?}", z);
        let result = z.eval(&ctx).unwrap();
        assert_eq!(result, 2);
    }
}
