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

#[macro_export]
macro_rules! int_impl {
    ($T:ident, $S:ident) => {
        $crate::impl_sym_val!($T, $S);

        $crate::impl_val_bin_ops!($T, Add, add);
        $crate::impl_val_bin_ops!($T, Sub, sub);
        $crate::impl_val_bin_ops!($T, Mul, mul);
        $crate::impl_val_bin_ops!($T, Div, div);
        $crate::impl_val_bin_ops!($T, BitAnd, bitand);
        $crate::impl_val_bin_ops!($T, BitOr, bitor);
        $crate::impl_val_bin_ops!($T, BitXor, bitxor);
        $crate::impl_val_bin_ops!($T, Shl, shl);
        $crate::impl_val_bin_ops!($T, Shr, shr);
        $crate::impl_val_bin_ops!($T, Rem, rem);
    };
}

#[macro_export]
macro_rules! uint_impl {
    ($T:ident, $S:ident) => {
        $crate::impl_sym_val!($T, $S);

        $crate::impl_val_bin_ops!($T, Add, add);
        $crate::impl_val_bin_ops!($T, Sub, sub);
        $crate::impl_val_bin_ops!($T, Mul, mul);
        $crate::impl_val_bin_ops!($T, Div, div);
        $crate::impl_val_bin_ops!($T, BitAnd, bitand);
        $crate::impl_val_bin_ops!($T, BitOr, bitor);
        $crate::impl_val_bin_ops!($T, BitXor, bitxor);
        $crate::impl_val_bin_ops!($T, Shl, shl);
        $crate::impl_val_bin_ops!($T, Shr, shr);
        $crate::impl_val_bin_ops!($T, Rem, rem);
    };
}

#[macro_export]
macro_rules! float_impl {
    ($T:ident, $S:ident) => {
        $crate::impl_sym_val!($T, $S);
        $crate::impl_val_bin_ops!($T, Add, add);
        $crate::impl_val_bin_ops!($T, Sub, sub);
        $crate::impl_val_bin_ops!($T, Mul, mul);
        $crate::impl_val_bin_ops!($T, Div, div);

        impl<C, E> $S<C, E>
        where
            C: $crate::SymCtx<$T>,
            E: $crate::SymExpr<$T>,
        {
            pub fn floor(&self) -> Self {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::floor)))
            }

            pub fn ceil(&self) -> Self {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::ceil)))
            }

            pub fn round(&self) -> Self {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::round)))
            }

            pub fn round_ties_even(&self) -> Self {
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::round_ties_even,
                )))
            }

            pub fn trunc(&self) -> Self {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::trunc)))
            }

            pub fn fract(&self) -> Self {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::fract)))
            }

            pub fn mul_add(&self, a: impl Into<$S<C, E>>, b: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _mul_add(x: ($T, $T, $T)) -> $T {
                    let (self_val, a_val, b_val) = x;
                    self_val.mul_add(a_val, b_val)
                }
                let a = a.into();
                let b = b.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), a, b), _mul_add)))
            }

            pub fn div_euclid(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _div_euclid(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.div_euclid(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _div_euclid,
                )))
            }

            pub fn rem_euclid(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _rem_euclid(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.rem_euclid(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _rem_euclid,
                )))
            }

            pub fn powi(&self, n: impl Into<$crate::SymI32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<i32>,
                E: $crate::SymExpr<i32>,
            {
                #[inline(always)]
                fn _powi(x: ($T, i32)) -> $T {
                    let (self_val, n) = x;
                    self_val.powi(n)
                }
                let n = n.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), n), _powi)))
            }

            pub fn powf(&self, n: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _powf(x: ($T, $T)) -> $T {
                    let (self_val, n_val) = x;
                    self_val.powf(n_val)
                }
                let n = n.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), n), _powf)))
            }

            pub fn sqrt(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::sqrt)))
            }

            pub fn exp(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::exp)))
            }

            pub fn exp2(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::exp2)))
            }

            pub fn ln(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::ln)))
            }

            pub fn log(&self, base: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _log(x: ($T, $T)) -> $T {
                    let (self_val, base_val) = x;
                    self_val.log(base_val)
                }
                let base = base.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), base), _log)))
            }

            pub fn log2(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::log2)))
            }

            pub fn log10(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::log10)))
            }

            pub fn cbrt(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::cbrt)))
            }

            pub fn hypot(&self, other: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _hypot(x: ($T, $T)) -> $T {
                    let (self_val, other_val) = x;
                    self_val.hypot(other_val)
                }
                let other = other.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), other), _hypot)))
            }

            pub fn sin(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::sin)))
            }

            pub fn cos(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::cos)))
            }

            pub fn tan(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::tan)))
            }

            pub fn asin(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::asin)))
            }

            pub fn acos(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::acos)))
            }

            pub fn atan(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::atan)))
            }

            pub fn atan2(&self, other: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _atan2(x: ($T, $T)) -> $T {
                    let (self_val, other_val) = x;
                    self_val.atan2(other_val)
                }
                let other = other.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), other), _atan2)))
            }

            // TODO: sin_cost, not support tuple value output now

            pub fn exp_m1(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::exp_m1)))
            }

            pub fn ln_1p(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::ln_1p)))
            }

            pub fn sinh(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::sinh)))
            }

            pub fn cosh(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::cosh)))
            }

            pub fn tanh(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::tanh)))
            }

            pub fn asinh(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::asinh)))
            }

            pub fn acosh(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::acosh)))
            }

            pub fn atanh(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::atanh)))
            }
        }
    };
}
