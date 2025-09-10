use crate::{Sym, SymCtx, SymExpr, SymFn, Value};


impl<T> Value for Option<T>
where
    T: Value,
{
    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Some(value) => value.display(f),
            None => write!(f, "None"),
        }
    }
}


impl<T, C, E> Sym<Option<T>, C, E>
where
    T: Value,
    C: SymCtx<Option<T>> + SymCtx<T> + SymCtx<bool>,
    E: SymExpr<Option<T>> + SymExpr<T> + SymExpr<bool>,
{
    fn is_some(&self) -> Sym<bool, C, E> {
        fn _is_some<X>(x: Option<X>) -> bool {
            x.is_some()
        }

        Sym::Expr(E::lift(SymFn::new(
            "is_some",
            (self.clone(),),
            _is_some::<T>,
        )))
    }

    fn is_none(&self) -> Sym<bool, C, E> {
        fn _is_none<X>(x: Option<X>) -> bool {
            x.is_none()
        }
        Sym::Expr(E::lift(SymFn::new(
            "is_none",
            (self.clone(),),
            _is_none::<T>,
        )))
    }

    fn unwrap(&self) -> Sym<T, C, E> {
        fn _unwrap<X>(x: Option<X>) -> X {
            x.unwrap()
        }
        Sym::Expr(E::lift(SymFn::new(
            "unwrap",
            (self.clone(),),
            _unwrap::<T>,
        )))
    }
}

