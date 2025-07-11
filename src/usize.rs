use crate::{Context, RcExpr, Sym, Value};

impl Value for usize {}

pub type SymUsize<C, E = RcExpr> = Sym<usize, C, E>;

#[cfg(test)]
mod tests {
    use crate::{SymCtx, SymValue};

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

        let w = z + k;
        let result = w.eval(&ctx).unwrap();
        assert_eq!(result, 8);

        println!("{:?}", w);
    }
}
