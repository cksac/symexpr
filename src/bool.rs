use std::collections::HashMap;

use symbol_table::Symbol;

use crate::{BoxExpr, Sym, impl_sym_val};

impl_sym_val!(bool);

pub type Bool<C = HashMap<Symbol, bool>, E = BoxExpr> = Sym<bool, C, E>;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{SymCtx, SymValue};

    use super::*;

    #[test]
    fn it_works() {
        let mut ctx = HashMap::new();
        ctx.bind("a", false);
        ctx.bind("b", true);

        let x: Sym<_, _, BoxExpr> = Bool::symbol("a");
        let r = x.eval(&ctx).unwrap();
        assert_eq!(r, false);

        let y: Sym<_, _, BoxExpr> = Bool::symbol("b");
        let y = y.eval(&ctx).unwrap();
        assert_eq!(y, true);
    }
}
