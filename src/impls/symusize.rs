use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(usize, SymUsize);

impl_val_bin_ops!(usize, Add, add);
impl_val_bin_ops!(usize, Sub, sub);
impl_val_bin_ops!(usize, Mul, mul);
impl_val_bin_ops!(usize, Div, div);

#[cfg(test)]
mod tests {
    use crate::ops::SymEq;
    use crate::{Context, SymCtx, SymValue};

    use super::*;

    type Usize = SymUsize;

    #[test]
    fn it_works() {
        let x = &Usize::symbol("a");
        let y = Usize::constant(4);
        let k = Usize::constant(2);

        let mut ctx = Context::default();
        ctx.set_symbol("a", 2usize);

        let result = x.eval(&ctx).unwrap();
        assert_eq!(result, 2);

        let result = y.eval(&ctx).unwrap();
        assert_eq!(result, 4);

        let b = x.eq(&y);
        let result = b.eval(&ctx).unwrap();
        assert_eq!(result, false);

        let z = x + y;
        let result = z.eval(&ctx).unwrap();
        assert_eq!(result, 6);

        let c = 3;
        let w = &c + z + 2 + k + 3;
        let result = w.eval(&ctx).unwrap();
        assert_eq!(result, 16);

        println!("{:?}", w);
    }
}
