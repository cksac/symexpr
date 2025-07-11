pub type Symbol = symbol_table::GlobalSymbol;

mod error;
pub use error::{Result, SymError};

mod sym;
pub use sym::*;

mod context;
pub use context::*;

pub mod ops;

mod macros;

mod impls;
pub use impls::*;

#[cfg(test)]
mod tests {
    use crate::ops::{SymEq, SymGe};
    use crate::{Context, SymCtx, SymUsize, SymValue};

    type Usize = SymUsize;

    #[test]
    fn it_works() {
        let x = &Usize::symbol("a");
        let y = &Usize::constant(4);
        let k = &Usize::constant(2);

        let mut ctx = Context::default();
        ctx.set_symbol("a", 2usize);

        let result = x.eval(&ctx).unwrap();
        assert_eq!(result, 2);

        let result = y.eval(&ctx).unwrap();
        assert_eq!(result, 4);

        let b = x.eq(y);
        let result = b.eval(&ctx).unwrap();
        assert_eq!(result, false);

        let b = x.ge(y);
        let result = b.eval(&ctx).unwrap();
        assert_eq!(result, false);

        let z = x + y;
        let result = z.eval(&ctx).unwrap();
        assert_eq!(result, 6);

        let c = 3;
        let w = c + z + 2 + k + 3;
        let result = w.eval(&ctx).unwrap();
        assert_eq!(result, 16);

        println!("{:?}", w);
    }
}
