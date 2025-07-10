#[macro_export]
macro_rules! std_bin_op {
    ($Sym:ident, $Op:ident, $Fn:ident, $Val:ident) => {
        impl<C, R> std::ops::$Op<R> for $crate::$Sym<C>
        where
            C: $crate::SymCtx<$Val>,
            R: $crate::SymValue<C, Value = $Val> + 'static,
        {
            type Output = $crate::$Sym<C>;
            fn $Fn(self, rhs: R) -> Self::Output {
                $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(self, rhs)))
            }
        }

        impl<C, R> std::ops::$Op<R> for &$crate::$Sym<C>
        where
            C: $crate::SymCtx<$Val>,
            R: $crate::SymValue<C, Value = $Val> + 'static,
        {
            type Output = $crate::$Sym<C>;
            fn $Fn(self, rhs: R) -> Self::Output {
                $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(self.cloned(), rhs)))
            }
        }

        impl<C> std::ops::$Op<$crate::$Sym<C>> for $Val
        where
            C: $crate::SymCtx<$Val>,
        {
            type Output = $crate::$Sym<C>;
            fn $Fn(self, rhs: $crate::$Sym<C>) -> Self::Output {
                $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(self, rhs)))
            }
        }

        impl<C> std::ops::$Op<&$crate::$Sym<C>> for $Val
        where
            C: $crate::SymCtx<$Val>,
        {
            type Output = $crate::$Sym<C>;
            fn $Fn(self, rhs: &$crate::$Sym<C>) -> Self::Output {
                $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(self, rhs.cloned())))
            }
        }

        impl<C> std::ops::$Op<$crate::$Sym<C>> for &$Val
        where
            C: $crate::SymCtx<$Val>,
        {
            type Output = $crate::$Sym<C>;
            fn $Fn(self, rhs: $crate::$Sym<C>) -> Self::Output {
                $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(*self, rhs)))
            }
        }

        impl<C> std::ops::$Op<&$crate::$Sym<C>> for &$Val
        where
            C: $crate::SymCtx<$Val>,
        {
            type Output = $crate::$Sym<C>;
            fn $Fn(self, rhs: &$crate::$Sym<C>) -> Self::Output {
                $crate::$Sym::Expr(Box::new($crate::expr::$Op::new(*self, rhs.cloned())))
            }
        }
    };
}
