use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

#[derive(Debug, Clone)]
pub struct Div<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    lhs: Sym<L, C, E>,
    rhs: Sym<R, C, E>,
}

impl<C, E, L, R> Div<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    pub fn new(lhs: Sym<L, C, E>, rhs: Sym<R, C, E>) -> Self {
        Div { lhs, rhs }
    }
}

impl<C, E, L, R, O> SymValue<C> for Div<C, E, L, R>
where
    L: Value,
    R: Value,
    O: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
    L: std::ops::Div<R, Output = O>,
{
    type Value = O;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs / rhs)
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.lhs.display(f)?;
        f.write_str(" / ")?;
        self.rhs.display(f)?;
        f.write_str(")")
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Div<Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Div<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn div(self, rhs: Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(Div::new(self, rhs)))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Div<Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Div<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn div(self, rhs: Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(Div::new(self.clone(), rhs)))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Div<&Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Div<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn div(self, rhs: &Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(Div::new(self, rhs.clone())))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Div<&Sym<RHS, C, E>> for &Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Div<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn div(self, rhs: &Sym<RHS, C, E>) -> Sym<OUT, C, E> {
        Sym::<OUT, C, E>::Expr(E::lift(Div::new(self.clone(), rhs.clone())))
    }
}

impl<C, E, LHS, RHS, OUT> std::ops::Div<RHS> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<OUT>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<OUT>,
    LHS: Value + std::ops::Div<RHS, Output = OUT>,
    LHS: Value,
    RHS: Value,
    OUT: Value,
{
    type Output = Sym<OUT, C, E>;
    fn div(self, rhs: RHS) -> Sym<OUT, C, E> {
        let rhs = Sym::<RHS, C, E>::constant(rhs);
        Sym::<OUT, C, E>::Expr(E::lift(Div::new(self, rhs)))
    }
}
