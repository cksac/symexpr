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

        pub type $Alias<C, E = $crate::RcExpr> = $crate::Sym<$Val, C, E>;
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
                $crate::Sym::Expr(E::wrap($crate::ops::$Op::new(self, rhs)))
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
                $crate::Sym::Expr(E::wrap($crate::ops::$Op::new(self, rhs.clone())))
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
                $crate::Sym::Expr(E::wrap($crate::ops::$Op::new(*self, rhs)))
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
                $crate::Sym::Expr(E::wrap($crate::ops::$Op::new(*self, rhs.clone())))
            }
        }
    };
}

// #[macro_export]
// macro_rules! std_bin_op {
//     ($Sym:ident, $Op:ident, $Fn:ident, $Val:ident) => {
//         impl<C, R> std::ops::$Op<R> for $crate::$Sym<C>
//         where
//             C: $crate::SymCtx<$Val>,
//             R: $crate::SymValue<C, Value = $Val> + 'static,
//         {
//             type Output = $crate::$Sym<C>;
//             fn $Fn(self, rhs: R) -> Self::Output {
//                 $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(self, rhs)))
//             }
//         }

//         impl<C, R> std::ops::$Op<R> for &$crate::$Sym<C>
//         where
//             C: $crate::SymCtx<$Val>,
//             R: $crate::SymValue<C, Value = $Val> + 'static,
//         {
//             type Output = $crate::$Sym<C>;
//             fn $Fn(self, rhs: R) -> Self::Output {
//                 $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(self.clone_box(), rhs)))
//             }
//         }

//         impl<C> std::ops::$Op<$crate::$Sym<C>> for $Val
//         where
//             C: $crate::SymCtx<$Val>,
//         {
//             type Output = $crate::$Sym<C>;
//             fn $Fn(self, rhs: $crate::$Sym<C>) -> Self::Output {
//                 $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(self, rhs)))
//             }
//         }

//         impl<C> std::ops::$Op<&$crate::$Sym<C>> for $Val
//         where
//             C: $crate::SymCtx<$Val>,
//         {
//             type Output = $crate::$Sym<C>;
//             fn $Fn(self, rhs: &$crate::$Sym<C>) -> Self::Output {
//                 $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(self, rhs.clone_box())))
//             }
//         }

//         impl<C> std::ops::$Op<$crate::$Sym<C>> for &$Val
//         where
//             C: $crate::SymCtx<$Val>,
//         {
//             type Output = $crate::$Sym<C>;
//             fn $Fn(self, rhs: $crate::$Sym<C>) -> Self::Output {
//                 $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(*self, rhs)))
//             }
//         }

//         impl<C> std::ops::$Op<&$crate::$Sym<C>> for &$Val
//         where
//             C: $crate::SymCtx<$Val>,
//         {
//             type Output = $crate::$Sym<C>;
//             fn $Fn(self, rhs: &$crate::$Sym<C>) -> Self::Output {
//                 $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(*self, rhs.clone_box())))
//             }
//         }
//     };
// }
