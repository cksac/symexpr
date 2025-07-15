use std::{fmt::Debug, ops::Deref, rc::Rc, sync::Arc};

use crate::{Context, Result, Symbol};

pub trait Value: Debug + Clone + 'static {}

pub trait SymValue<C>: Debug {
    type Value: Value;
    fn eval(&self, ctx: &C) -> Result<Self::Value>;
}

pub trait SymCtx<T>: Debug + 'static {
    fn get_symbol(&self, symbol: Symbol) -> Result<T>;
    fn set_symbol(&mut self, symbol: impl AsRef<str>, value: T);
    fn del_symbol(&mut self, symbol: impl AsRef<str>);
}

pub trait SymExpr<T>: Debug + 'static {
    type Expr<C: SymCtx<T>>: Debug + Clone + Deref<Target = dyn SymValue<C, Value = T>>;

    fn lift<C, E>(expr: E) -> Self::Expr<C>
    where
        C: SymCtx<T>,
        E: SymValue<C, Value = T> + 'static;
}

#[derive(Debug, Clone)]
pub struct RcExpr;

impl<T> SymExpr<T> for RcExpr
where
    T: Value,
{
    type Expr<C: SymCtx<T>> = Rc<dyn SymValue<C, Value = T>>;

    fn lift<C, E>(expr: E) -> Self::Expr<C>
    where
        C: SymCtx<T>,
        E: SymValue<C, Value = T> + 'static,
    {
        Rc::new(expr)
    }
}

#[derive(Debug, Clone)]
pub struct ArcExpr;

impl<T> SymExpr<T> for ArcExpr
where
    T: Value,
{
    type Expr<C: SymCtx<T>> = Arc<dyn SymValue<C, Value = T>>;

    fn lift<C, E>(expr: E) -> Self::Expr<C>
    where
        C: SymCtx<T>,
        E: SymValue<C, Value = T> + 'static,
    {
        Arc::new(expr)
    }
}

#[derive(Debug)]
pub enum Sym<T, C = Context, E = RcExpr>
where
    T: Value,
    C: SymCtx<T>,
    E: SymExpr<T>,
{
    Symbol(Symbol),
    Const(T),
    Expr(E::Expr<C>),
}

impl<T, C, E> Clone for Sym<T, C, E>
where
    T: Value,
    C: SymCtx<T>,
    E: SymExpr<T>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Symbol(s) => Self::Symbol(*s),
            Self::Const(v) => Self::Const(v.clone()),
            Self::Expr(e) => Self::Expr(e.clone()),
        }
    }
}

impl<T, C, E> Sym<T, C, E>
where
    T: Value,
    C: SymCtx<T>,
    E: SymExpr<T>,
{
    pub fn symbol(name: impl AsRef<str>) -> Self {
        Self::Symbol(Symbol::new(name))
    }

    pub fn constant(value: T) -> Self {
        Self::Const(value)
    }
}

impl<T, C, E> SymValue<C> for Sym<T, C, E>
where
    T: Value,
    C: SymCtx<T>,
    E: SymExpr<T>,
{
    type Value = T;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        match self {
            Sym::Symbol(s) => ctx.get_symbol(*s),
            Sym::Const(v) => Ok(v.clone()),
            Sym::Expr(e) => e.eval(ctx),
        }
    }
}

impl<T> Value for Option<T> where T: Value {}
