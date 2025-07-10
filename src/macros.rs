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
    ($Sym:ident, $SymCtx:ident, $Val:ident) => {
        impl $crate::Value for $Val {}

        #[derive(Debug)]
        pub enum $Sym<C>
        where
            C: $crate::SymCtx<$Val>,
        {
            Const($Val),
            Symbol($crate::Symbol),
            Expr(Box<dyn $crate::SymValue<C, Value = $Val>>),
        }

        #[derive(Debug, Default)]
        pub struct $SymCtx {
            bindings: std::collections::HashMap<$crate::Symbol, $Val>,
        }

        impl<C> $crate::SymValue<C> for $Sym<C>
        where
            C: $crate::SymCtx<$Val>,
        {
            type Value = $Val;
            fn eval(&self, ctx: &C) -> $crate::Result<Self::Value> {
                match self {
                    $Sym::Const(v) => Ok(*v),
                    $Sym::Symbol(s) => ctx.get(*s),
                    $Sym::Expr(e) => e.eval(ctx),
                }
            }

            fn cloned(&self) -> Box<dyn $crate::SymValue<C, Value = Self::Value>> {
                match self {
                    $Sym::Const(v) => Box::new(*v),
                    $Sym::Symbol(s) => Box::new($Sym::Symbol(*s)),
                    $Sym::Expr(e) => e.cloned(),
                }
            }
        }

        impl<C> $crate::SymValue<C> for $Val {
            type Value = $Val;
            fn eval(&self, _ctx: &C) -> $crate::Result<Self::Value> {
                Ok(*self)
            }

            fn cloned(&self) -> Box<dyn $crate::SymValue<C, Value = Self::Value>> {
                Box::new(*self)
            }
        }

        impl<C> $Sym<C>
        where
            C: $crate::SymCtx<$Val>,
        {
            pub fn symbol(name: impl AsRef<str>) -> Self {
                $Sym::Symbol($crate::Symbol::new(name))
            }
        }

        impl $SymCtx {
            pub fn new() -> Self {
                Self {
                    bindings: std::collections::HashMap::new(),
                }
            }
        }

        impl $crate::SymCtx<$Val> for $SymCtx {
            fn get(&self, symbol: $crate::Symbol) -> $crate::Result<$Val> {
                self.bindings
                    .get(&symbol)
                    .copied()
                    .ok_or($crate::SymError::SymbolNotFound(symbol))
            }

            fn bind(&mut self, symbol: impl AsRef<str>, value: $Val) {
                self.bindings.insert($crate::Symbol::new(symbol), value);
            }
        }
    };
}
