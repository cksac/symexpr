use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

pub trait SymOr<C, E, R>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
{
    fn or(self, rhs: R) -> Sym<bool, C, E>;
}

#[derive(Debug, Clone)]
pub struct Or<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    lhs: Sym<L, C, E>,
    rhs: Sym<R, C, E>,
}

impl<C, E, L, R> Or<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
{
    pub fn new(lhs: Sym<L, C, E>, rhs: Sym<R, C, E>) -> Self {
        Or { lhs, rhs }
    }
}

impl<C, E> SymValue<C> for Or<C, E, bool, bool>
where
    bool: Value,
    C: SymCtx<bool>,
    E: SymExpr<bool>,
{
    type Value = bool;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs || rhs)
    }
}

impl<C, E> SymOr<C, E, Sym<bool, C, E>> for Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    bool: Value,
{
    fn or(self, rhs: Sym<bool, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Or::new(self, rhs)))
    }
}

impl<C, E> SymOr<C, E, &Sym<bool, C, E>> for Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    bool: Value,
{
    fn or(self, rhs: &Sym<bool, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Or::new(self, rhs.clone())))
    }
}

impl<C, E> SymOr<C, E, Sym<bool, C, E>> for &Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    bool: Value,
{
    fn or(self, rhs: Sym<bool, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Or::new(self.clone(), rhs)))
    }
}

impl<C, E> SymOr<C, E, &Sym<bool, C, E>> for &Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    bool: Value,
{
    fn or(self, rhs: &Sym<bool, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Or::new(self.clone(), rhs.clone())))
    }
}

impl<C, E, RHS> SymOr<C, E, RHS> for Sym<bool, C, E>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
    RHS: Value + Into<bool>,
{
    fn or(self, rhs: RHS) -> Sym<bool, C, E> {
        let rhs = Sym::<bool, C, E>::constant(rhs.into());
        Sym::<bool, C, E>::Expr(E::lift(Or::new(self, rhs)))
    }
}

impl<C, E, LHS> Sym<LHS, C, E>
where
    LHS: Value,
    C: SymCtx<LHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<bool>,
{
    #[inline(always)]
    pub fn or<RHS>(&self, rhs: RHS) -> Sym<bool, C, E>
    where
        for<'a> &'a Self: SymOr<C, E, RHS>,
    {
        SymOr::or(self, rhs)
    }
}
