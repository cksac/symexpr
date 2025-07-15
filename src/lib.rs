pub type Symbol = symbol_table::GlobalSymbol;

mod error;
pub use error::{Result, SymError};

mod sym;
pub use sym::*;

mod context;
pub use context::*;

pub mod ops;

mod symfn;
pub use symfn::*;

mod impls;
pub use impls::*;

mod macros;

#[cfg(test)]
mod tests {
    use crate::{Context, SymCtx, SymF32, SymUsize, SymValue};
    type Usize = SymUsize;
    type F32 = SymF32;

    #[test]
    fn test_basic() {
        // Symbolic variables and constants
        let x = &Usize::symbol("a");
        let y = &Usize::constant(4);
        let k = &Usize::constant(2);

        // Context for evaluation
        let mut ctx = Context::default();
        ctx.set_symbol("a", 2usize);

        // Evaluate symbolic values
        assert_eq!(x.eval(&ctx).unwrap(), 2);
        assert_eq!(y.eval(&ctx).unwrap(), 4);

        // Symbolic comparisons
        let b = x.eq(y);
        assert!(!b.eval(&ctx).unwrap());
        let b = x.ge(y);
        assert!(!b.eval(&ctx).unwrap());

        // Symbolic arithmetic
        let z = x + y;
        assert_eq!(z.eval(&ctx).unwrap(), 6);

        // Mixed expressions
        let c = 3;
        let w = c + z + 2 + k + 3;
        assert_eq!(w.eval(&ctx).unwrap(), 16);

        // Symbolic math functions (e.g., floor)
        let xf = F32::symbol("f");
        ctx.set_symbol("f", 3.7f32);
        let floor_xf = xf.floor();
        assert_eq!(floor_xf.eval(&ctx).unwrap(), 3.0);
    }
}
