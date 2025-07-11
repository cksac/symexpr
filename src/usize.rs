use crate::{impl_sym_val, impl_val_bin_ops};

impl_sym_val!(usize, SymUsize);

impl_val_bin_ops!(usize, Add, add);

#[cfg(test)]
mod tests {
    use crate::{Context, SymCtx, SymValue};

    use super::*;

    type Ctx = Context<usize>;
    type Usize = SymUsize<Ctx>;

    #[test]
    fn it_works() {
        let x = Usize::symbol("a");
        let y = Usize::value(4usize);
        let k = Usize::value(2usize);

        let mut ctx = Ctx::new();
        ctx.bind("a", 2);

        let result = x.eval(&ctx).unwrap();
        assert_eq!(result, 2);

        let result = y.eval(&ctx).unwrap();
        assert_eq!(result, 4);

        let z = x + y;
        let result = z.eval(&ctx).unwrap();
        assert_eq!(result, 6);

        let c = 3;
        let w = &c + 2 + z + k + 3;
        let result = w.eval(&ctx).unwrap();
        assert_eq!(result, 16);

        println!("{:?}", w);
    }
}
