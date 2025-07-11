use std::{collections::HashMap, fmt::Debug, ops::Deref, rc::Rc, sync::Arc};

use crate::{Result, SymError, Symbol};

pub trait Value: Debug + Clone + 'static {}

pub trait SymValue: Debug {
    type Value: Value;
    type Context: SymCtx<Self::Value>;
    fn eval(&self, ctx: &Self::Context) -> Result<Self::Value>;
}

pub trait SymCtx<T>: Debug + 'static {
    fn get(&self, symbol: Symbol) -> Result<T>;
    fn bind(&mut self, symbol: impl AsRef<str>, value: T);
}

impl<T> SymCtx<T> for HashMap<Symbol, T>
where
    T: Value,
{
    fn get(&self, symbol: Symbol) -> Result<T> {
        self.get(&symbol)
            .cloned()
            .ok_or_else(|| SymError::SymbolNotFound(symbol))
    }

    fn bind(&mut self, symbol: impl AsRef<str>, value: T) {
        let symbol = Symbol::new(symbol);
        self.insert(symbol, value);
    }
}

pub type Context<T> = HashMap<Symbol, T>;

pub trait SymExpr<T>: Debug + 'static {
    type Expr<C: SymCtx<T>>: Debug + Clone + Deref<Target = dyn SymValue<Value = T, Context = C>>;
    fn wrap<C, E>(expr: E) -> Self::Expr<C>
    where
        C: SymCtx<T>,
        E: SymValue<Value = T, Context = C> + 'static;
}

#[derive(Debug, Clone)]
pub struct RcExpr;

impl<T> SymExpr<T> for RcExpr
where
    T: Value,
{
    type Expr<C: SymCtx<T>> = Rc<dyn SymValue<Value = T, Context = C>>;
    fn wrap<C, E>(expr: E) -> Self::Expr<C>
    where
        C: SymCtx<T>,
        E: SymValue<Value = T, Context = C> + 'static,
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
    type Expr<C: SymCtx<T>> = Arc<dyn SymValue<Value = T, Context = C>>;
    fn wrap<C, E>(expr: E) -> Self::Expr<C>
    where
        C: SymCtx<T>,
        E: SymValue<Value = T, Context = C> + 'static,
    {
        Arc::new(expr)
    }
}

#[derive(Debug)]
pub enum Sym<T, C, E = RcExpr>
where
    T: Value,
    C: SymCtx<T>,
    E: SymExpr<T>,
{
    Symbol(Symbol),
    Const(T),
    Expr(E::Expr<C>),
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

    pub fn value(value: impl Into<T>) -> Self {
        Self::Const(value.into())
    }
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

impl<T, C, E> SymValue for Sym<T, C, E>
where
    T: Value,
    C: SymCtx<T>,
    E: SymExpr<T>,
{
    type Value = T;
    type Context = C;

    fn eval(&self, ctx: &Self::Context) -> Result<Self::Value> {
        match self {
            Self::Symbol(s) => ctx.get(*s),
            Self::Const(v) => Ok(v.clone()),
            Self::Expr(e) => e.eval(ctx),
        }
    }
}
