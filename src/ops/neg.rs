use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

#[derive(Debug, Clone)]
pub struct Neg<C, E, L>
where
    L: Value,
    C: SymCtx<L>,
    E: SymExpr<L>,
{
    x: Sym<L, C, E>,
}

impl<C, E, L> Neg<C, E, L>
where
    L: Value,
    C: SymCtx<L>,
    E: SymExpr<L>,
{
    pub fn new(x: Sym<L, C, E>) -> Self {
        Neg { x }
    }
}

impl<C, E, L, O> SymValue<C> for Neg<C, E, L>
where
    L: Value,
    O: Value,
    C: SymCtx<L> + SymCtx<O>,
    E: SymExpr<L> + SymExpr<O>,
    L: std::ops::Neg<Output = O>,
{
    type Value = O;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let x = self.x.eval(ctx)?;
        Ok(-x)
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("-")?;
        self.x.display(f)
    }
}

impl<C, E, L, O> std::ops::Neg for Sym<L, C, E>
where
    C: SymCtx<L> + SymCtx<O>,
    E: SymExpr<L> + SymExpr<O>,
    L: Value + std::ops::Neg<Output = O>,
    O: Value,
{
    type Output = Sym<O, C, E>;
    fn neg(self) -> Self::Output {
        Sym::<O, C, E>::Expr(E::lift(Neg::new(self)))
    }
}

impl<C, E, L, O> std::ops::Neg for &Sym<L, C, E>
where
    C: SymCtx<L> + SymCtx<O>,
    E: SymExpr<L> + SymExpr<O>,
    L: Value + std::ops::Neg<Output = O>,
    O: Value,
{
    type Output = Sym<O, C, E>;
    fn neg(self) -> Self::Output {
        Sym::<O, C, E>::Expr(E::lift(Neg::new(self.clone())))
    }
}
