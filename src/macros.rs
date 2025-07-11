#[macro_export]
macro_rules! impl_sym_val {
    ($Val:ident, $Alias:ident) => {
        impl $crate::Value for $Val {}

        impl<C> $crate::SymValue<C> for $Val {
            type Value = $Val;
            fn eval(&self, _ctx: &C) -> $crate::Result<Self::Value> {
                Ok(*self)
            }
        }

        #[allow(dead_code)]
        pub type $Alias<C = $crate::Context, E = $crate::RcExpr> = $crate::Sym<$Val, C, E>;
    };
}

#[macro_export]
macro_rules! impl_val_bin_ops {
    ($LHS:ident, $Op:ident, $Fn:ident) => {
        impl<C, E, RHS, OUT> std::ops::$Op<$crate::Sym<RHS, C, E>> for $LHS
        where
            RHS: $crate::Value,
            OUT: $crate::Value,
            C: $crate::SymCtx<$LHS> + $crate::SymCtx<RHS> + $crate::SymCtx<OUT>,
            E: $crate::SymExpr<$LHS> + $crate::SymExpr<RHS> + $crate::SymExpr<OUT>,
            $LHS: std::ops::$Op<RHS, Output = OUT>,
        {
            type Output = $crate::Sym<OUT, C, E>;

            fn $Fn(self, rhs: $crate::Sym<RHS, C, E>) -> Self::Output {
                let lhs = $crate::Sym::<$LHS, C, E>::constant(self);
                $crate::Sym::Expr(E::lift($crate::ops::$Op::new(lhs, rhs)))
            }
        }

        impl<C, E, RHS, OUT> std::ops::$Op<&$crate::Sym<RHS, C, E>> for $LHS
        where
            RHS: $crate::Value,
            OUT: $crate::Value,
            C: $crate::SymCtx<$LHS> + $crate::SymCtx<RHS> + $crate::SymCtx<OUT>,
            E: $crate::SymExpr<$LHS> + $crate::SymExpr<RHS> + $crate::SymExpr<OUT>,
            $LHS: std::ops::$Op<RHS, Output = OUT>,
        {
            type Output = $crate::Sym<OUT, C, E>;

            fn $Fn(self, rhs: &$crate::Sym<RHS, C, E>) -> Self::Output {
                let lhs = $crate::Sym::<$LHS, C, E>::constant(self);
                $crate::Sym::Expr(E::lift($crate::ops::$Op::new(lhs, rhs.clone())))
            }
        }

        impl<C, E, RHS, OUT> std::ops::$Op<$crate::Sym<RHS, C, E>> for &$LHS
        where
            RHS: $crate::Value,
            OUT: $crate::Value,
            C: $crate::SymCtx<$LHS> + $crate::SymCtx<RHS> + $crate::SymCtx<OUT>,
            E: $crate::SymExpr<$LHS> + $crate::SymExpr<RHS> + $crate::SymExpr<OUT>,
            $LHS: std::ops::$Op<RHS, Output = OUT>,
        {
            type Output = $crate::Sym<OUT, C, E>;

            fn $Fn(self, rhs: $crate::Sym<RHS, C, E>) -> Self::Output {
                let lhs = $crate::Sym::<$LHS, C, E>::constant(*self);
                $crate::Sym::Expr(E::lift($crate::ops::$Op::new(lhs, rhs)))
            }
        }

        impl<C, E, RHS, OUT> std::ops::$Op<&$crate::Sym<RHS, C, E>> for &$LHS
        where
            RHS: $crate::Value,
            OUT: $crate::Value,
            C: $crate::SymCtx<$LHS> + $crate::SymCtx<RHS> + $crate::SymCtx<OUT>,
            E: $crate::SymExpr<$LHS> + $crate::SymExpr<RHS> + $crate::SymExpr<OUT>,
            $LHS: std::ops::$Op<RHS, Output = OUT>,
        {
            type Output = $crate::Sym<OUT, C, E>;

            fn $Fn(self, rhs: &$crate::Sym<RHS, C, E>) -> Self::Output {
                let lhs = $crate::Sym::<$LHS, C, E>::constant(*self);
                $crate::Sym::Expr(E::lift($crate::ops::$Op::new(lhs, rhs.clone())))
            }
        }
    };
}
