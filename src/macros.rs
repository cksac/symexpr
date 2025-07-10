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

#[macro_export]
macro_rules! define_sym_val {
    ($Sym:ident, $Val:ident) => {
        impl $crate::Value for $Val {}

        pub type $Sym<C> = $crate::Sym<$Val, C>;

        impl<C> $crate::SymValue<C> for $Val {
            type Value = $Val;
            fn eval(&self, _ctx: &C) -> $crate::Result<Self::Value> {
                Ok(*self)
            }

            fn cloned(&self) -> Box<dyn $crate::SymValue<C, Value = Self::Value>> {
                Box::new(*self)
            }
        }
    };
}
