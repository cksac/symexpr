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

        impl<C, E> From<$Val> for $Alias<C, E>
        where
            C: $crate::SymCtx<$Val>,
            E: $crate::SymExpr<$Val>,
        {
            fn from(value: $Val) -> Self {
                $Alias::constant(value)
            }
        }
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
                match rhs {
                    $crate::Sym::<RHS, C, E>::Const(rhs_val) => {
                        let out: OUT = std::ops::$Op::<RHS>::$Fn(self, rhs_val);
                        $crate::Sym::<OUT, C, E>::constant(out)
                    }
                    _ => {
                        let lhs = $crate::Sym::<$LHS, C, E>::constant(self);
                        $crate::Sym::Expr(E::lift($crate::ops::$Op::new(lhs, rhs)))
                    }
                }
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
                match rhs {
                    $crate::Sym::<RHS, C, E>::Const(rhs_val) => {
                        let out: OUT = std::ops::$Op::<RHS>::$Fn(self, rhs_val.clone());
                        $crate::Sym::<OUT, C, E>::constant(out)
                    }
                    _ => {
                        let lhs = $crate::Sym::<$LHS, C, E>::constant(self);
                        $crate::Sym::Expr(E::lift($crate::ops::$Op::new(lhs, rhs.clone())))
                    }
                }
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
                match rhs {
                    $crate::Sym::<RHS, C, E>::Const(rhs_val) => {
                        let out: OUT = std::ops::$Op::<RHS>::$Fn(self.clone(), rhs_val);
                        $crate::Sym::<OUT, C, E>::constant(out)
                    }
                    _ => {
                        let lhs = $crate::Sym::<$LHS, C, E>::constant(self.clone());
                        $crate::Sym::Expr(E::lift($crate::ops::$Op::new(lhs, rhs)))
                    }
                }
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
                match rhs {
                    $crate::Sym::<RHS, C, E>::Const(rhs_val) => {
                        let out: OUT = std::ops::$Op::<RHS>::$Fn(self.clone(), rhs_val.clone());
                        $crate::Sym::<OUT, C, E>::constant(out)
                    }
                    _ => {
                        let lhs = $crate::Sym::<$LHS, C, E>::constant(self.clone());
                        $crate::Sym::Expr(E::lift($crate::ops::$Op::new(lhs, rhs.clone())))
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! int_impl {
    ($T:ident, $S:ident, $UT:ident, $US:ident) => {
        $crate::integer_common_impl!($T, $S);

        impl<C, E> $S<C, E>
        where
            C: $crate::SymCtx<$T>,
            E: $crate::SymExpr<$T>,
        {
            pub fn cast_unsigned(&self) -> $crate::$US<C, E>
            where
                C: $crate::SymCtx<$UT>,
                E: $crate::SymExpr<$UT>,
            {
                $crate::$US::<C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::cast_unsigned,
                )))
            }

            pub fn checked_add_unsigned(
                &self,
                rhs: impl Into<$crate::$US<C, E>>,
            ) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>> + $crate::SymCtx<$UT>,
                E: $crate::SymExpr<Option<$T>> + $crate::SymExpr<$UT>,
            {
                #[inline(always)]
                fn _checked_add_unsigned(x: ($T, $UT)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_add_unsigned(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_add_unsigned,
                )))
            }

            pub fn saturating_add_unsigned(&self, rhs: impl Into<$crate::$US<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<$UT>,
                E: $crate::SymExpr<$UT>,
            {
                #[inline(always)]
                fn _saturating_add_unsigned(x: ($T, $UT)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.saturating_add_unsigned(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _saturating_add_unsigned,
                )))
            }

            pub fn checked_sub_unsigned(
                &self,
                rhs: impl Into<$crate::$US<C, E>>,
            ) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>> + $crate::SymCtx<$UT>,
                E: $crate::SymExpr<Option<$T>> + $crate::SymExpr<$UT>,
            {
                #[inline(always)]
                fn _checked_sub_unsigned(x: ($T, $UT)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_sub_unsigned(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_sub_unsigned,
                )))
            }

            pub fn saturating_sub_unsigned(&self, rhs: impl Into<$crate::$US<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<$UT>,
                E: $crate::SymExpr<$UT>,
            {
                #[inline(always)]
                fn _saturating_sub_unsigned(x: ($T, $UT)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.saturating_sub_unsigned(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _saturating_sub_unsigned,
                )))
            }

            pub fn wrapping_add_unsigned(&self, rhs: impl Into<$crate::$US<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<$UT>,
                E: $crate::SymExpr<$UT>,
            {
                #[inline(always)]
                fn _wrapping_add_unsigned(x: ($T, $UT)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_add_unsigned(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_add_unsigned,
                )))
            }

            pub fn wrapping_sub_unsigned(&self, rhs: impl Into<$crate::$US<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<$UT>,
                E: $crate::SymExpr<$UT>,
            {
                #[inline(always)]
                fn _wrapping_sub_unsigned(x: ($T, $UT)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_sub_unsigned(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_sub_unsigned,
                )))
            }

            pub fn checked_abs(&self) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::checked_abs,
                )))
            }

            pub fn unsigned_abs(&self) -> $crate::$US<C, E>
            where
                C: $crate::SymCtx<$UT>,
                E: $crate::SymExpr<$UT>,
            {
                $crate::$US::<C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::unsigned_abs,
                )))
            }

            pub fn abs_diff(&self, rhs: impl Into<$S<C, E>>) -> $crate::$US<C, E>
            where
                C: $crate::SymCtx<$UT>,
                E: $crate::SymExpr<$UT>,
            {
                #[inline(always)]
                fn _abs_diff(x: ($T, $T)) -> $UT {
                    let (self_val, rhs_val) = x;
                    self_val.abs_diff(rhs_val)
                }
                let rhs = rhs.into();
                $crate::$US::Expr(E::lift($crate::SymFn::new((self.clone(), rhs), _abs_diff)))
            }

            pub fn abs(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::abs)))
            }

            pub fn wrapping_abs(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::wrapping_abs,
                )))
            }

            pub fn saturating_abs(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::saturating_abs,
                )))
            }

            pub fn saturating_neg(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::saturating_neg,
                )))
            }

            pub fn signum(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::signum)))
            }

            pub fn is_positive(&self) -> $crate::SymBool<C, E>
            where
                C: $crate::SymCtx<bool>,
                E: $crate::SymExpr<bool>,
            {
                $crate::SymBool::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::is_positive,
                )))
            }

            pub fn is_negative(&self) -> $crate::SymBool<C, E>
            where
                C: $crate::SymCtx<bool>,
                E: $crate::SymExpr<bool>,
            {
                $crate::SymBool::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::is_negative,
                )))
            }
        }
    };
}

#[macro_export]
macro_rules! uint_impl {
    ($T:ident, $S:ident, $ST:ident, $SS:ident) => {
        $crate::integer_common_impl!($T, $S);

        impl<C, E> $S<C, E>
        where
            C: $crate::SymCtx<$T>,
            E: $crate::SymExpr<$T>,
        {
            pub fn cast_signed(&self) -> $crate::$SS<C, E>
            where
                C: $crate::SymCtx<$ST>,
                E: $crate::SymExpr<$ST>,
            {
                $crate::$SS::<C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::cast_signed,
                )))
            }

            pub fn checked_add_signed(
                &self,
                rhs: impl Into<$crate::$SS<C, E>>,
            ) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>> + $crate::SymCtx<$ST>,
                E: $crate::SymExpr<Option<$T>> + $crate::SymExpr<$ST>,
            {
                #[inline(always)]
                fn _checked_add_signed(x: ($T, $ST)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_add_signed(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_add_signed,
                )))
            }

            pub fn saturating_add_signed(&self, rhs: impl Into<$crate::$SS<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<$ST>,
                E: $crate::SymExpr<$ST>,
            {
                #[inline(always)]
                fn _saturating_add_signed(x: ($T, $ST)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.saturating_add_signed(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _saturating_add_signed,
                )))
            }

            pub fn wrapping_add_signed(&self, rhs: impl Into<$crate::$SS<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<$ST>,
                E: $crate::SymExpr<$ST>,
            {
                #[inline(always)]
                fn _wrapping_add_signed(x: ($T, $ST)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_add_signed(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_add_signed,
                )))
            }

            pub fn abs_diff(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _abs_diff(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.abs_diff(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), rhs), _abs_diff)))
            }

            pub fn div_ceil(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _div_ceil(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.div_ceil(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), rhs), _div_ceil)))
            }

            pub fn checked_next_multiple_of(
                &self,
                rhs: impl Into<$S<C, E>>,
            ) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                #[inline(always)]
                fn _checked_next_multiple_of(x: ($T, $T)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_next_multiple_of(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_next_multiple_of,
                )))
            }

            pub fn next_multiple_of(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _next_multiple_of(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.next_multiple_of(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _next_multiple_of,
                )))
            }

            pub fn is_multiple_of(&self, rhs: impl Into<$S<C, E>>) -> $crate::SymBool<C, E>
            where
                C: $crate::SymCtx<bool>,
                E: $crate::SymExpr<bool>,
            {
                #[inline(always)]
                fn _is_multiple_of(x: ($T, $T)) -> bool {
                    let (self_val, rhs_val) = x;
                    self_val.is_multiple_of(rhs_val)
                }
                let rhs = rhs.into();
                $crate::SymBool::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _is_multiple_of,
                )))
            }

            pub fn checked_next_power_of_two(&self) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::checked_next_power_of_two,
                )))
            }

            pub fn is_power_of_two(&self) -> $crate::SymBool<C, E>
            where
                C: $crate::SymCtx<bool>,
                E: $crate::SymExpr<bool>,
            {
                $crate::SymBool::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::is_power_of_two,
                )))
            }

            pub fn next_power_of_two(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::next_power_of_two,
                )))
            }
        }
    };
}

#[macro_export]
macro_rules! integer_common_impl {
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

        impl<C, E> $S<C, E>
        where
            C: $crate::SymCtx<$T>,
            E: $crate::SymExpr<$T>,
        {
            pub fn count_ones(&self) -> $crate::SymU32<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                $crate::SymU32::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::count_ones)))
            }

            pub fn count_zeros(&self) -> $crate::SymU32<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                $crate::SymU32::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::count_zeros,
                )))
            }

            pub fn leading_zeros(&self) -> $crate::SymU32<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                $crate::SymU32::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::leading_zeros,
                )))
            }

            pub fn trailing_zeros(&self) -> $crate::SymU32<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                $crate::SymU32::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::trailing_zeros,
                )))
            }

            pub fn leading_ones(&self) -> $crate::SymU32<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                $crate::SymU32::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::leading_ones,
                )))
            }

            pub fn trailing_ones(&self) -> $crate::SymU32<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                $crate::SymU32::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::trailing_ones,
                )))
            }

            pub fn rotate_left(&self, rhs: impl Into<$crate::SymU32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _rotate_left(x: ($T, u32)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.rotate_left(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _rotate_left,
                )))
            }

            pub fn rotate_right(&self, rhs: impl Into<$crate::SymU32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _rotate_right(x: ($T, u32)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.rotate_right(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _rotate_right,
                )))
            }

            pub fn swap_bytes(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::swap_bytes)))
            }

            pub fn reverse_bits(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::reverse_bits,
                )))
            }

            pub fn from_be(x: impl Into<$S<C, E>>) -> $S<C, E> {
                let x = x.into();
                $S::Expr(E::lift($crate::SymFn::new((x,), $T::from_be)))
            }

            pub fn from_le(x: impl Into<$S<C, E>>) -> $S<C, E> {
                let x = x.into();
                $S::Expr(E::lift($crate::SymFn::new((x,), $T::from_le)))
            }

            pub fn to_be(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::to_be)))
            }

            pub fn to_le(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::to_le)))
            }

            pub fn checked_add(&self, rhs: impl Into<$S<C, E>>) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                #[inline(always)]
                fn _checked_add(x: ($T, $T)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_add(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_add,
                )))
            }

            pub unsafe fn unchecked_add(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _unchecked_add(x: ($T, $T)) -> $T {
                    unsafe {
                        let (self_val, rhs_val) = x;
                        self_val.unchecked_add(rhs_val)
                    }
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _unchecked_add,
                )))
            }

            pub fn checked_sub(&self, rhs: impl Into<$S<C, E>>) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                #[inline(always)]
                fn _checked_sub(x: ($T, $T)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_sub(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_sub,
                )))
            }

            pub unsafe fn unchecked_sub(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _unchecked_sub(x: ($T, $T)) -> $T {
                    unsafe {
                        let (self_val, rhs_val) = x;
                        self_val.unchecked_sub(rhs_val)
                    }
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _unchecked_sub,
                )))
            }

            pub fn checked_mul(&self, rhs: impl Into<$S<C, E>>) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                #[inline(always)]
                fn _checked_mul(x: ($T, $T)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_mul(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_mul,
                )))
            }

            pub unsafe fn unchecked_mul(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _unchecked_mul(x: ($T, $T)) -> $T {
                    unsafe {
                        let (self_val, rhs_val) = x;
                        self_val.unchecked_mul(rhs_val)
                    }
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _unchecked_mul,
                )))
            }

            pub fn checked_div(&self, rhs: impl Into<$S<C, E>>) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                #[inline(always)]
                fn _checked_div(x: ($T, $T)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_div(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_div,
                )))
            }

            pub fn checked_div_euclid(
                &self,
                rhs: impl Into<$S<C, E>>,
            ) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                #[inline(always)]
                fn _checked_div_euclid(x: ($T, $T)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_div_euclid(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_div_euclid,
                )))
            }

            pub fn checked_rem(&self, rhs: impl Into<$S<C, E>>) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                #[inline(always)]
                fn _checked_rem(x: ($T, $T)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_rem(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_rem,
                )))
            }

            pub fn checked_rem_euclid(
                &self,
                rhs: impl Into<$S<C, E>>,
            ) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                #[inline(always)]
                fn _checked_rem_euclid(x: ($T, $T)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_rem_euclid(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_rem_euclid,
                )))
            }

            pub fn unbounded_shl(&self, rhs: impl Into<$crate::SymU32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _unbounded_shl(x: ($T, u32)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.unbounded_shl(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _unbounded_shl,
                )))
            }

            pub fn unbounded_shr(&self, rhs: impl Into<$crate::SymU32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _unbounded_shr(x: ($T, u32)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.unbounded_shr(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _unbounded_shr,
                )))
            }

            pub fn saturating_add(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _saturating_add(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.saturating_add(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _saturating_add,
                )))
            }

            pub fn saturating_sub(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _saturating_sub(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.saturating_sub(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _saturating_sub,
                )))
            }

            pub fn saturating_mul(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _saturating_mul(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.saturating_mul(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _saturating_mul,
                )))
            }

            pub fn saturating_div(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _saturating_div(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.saturating_div(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _saturating_div,
                )))
            }

            pub fn checked_pow(
                &self,
                n: impl Into<$crate::SymU32<C, E>>,
            ) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>> + $crate::SymCtx<u32>,
                E: $crate::SymExpr<Option<$T>> + $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _checked_pow(x: ($T, u32)) -> Option<$T> {
                    let (self_val, n_val) = x;
                    self_val.checked_pow(n_val)
                }
                let n = n.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), n),
                    _checked_pow,
                )))
            }

            pub fn saturating_pow(&self, n: impl Into<$crate::SymU32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _saturating_pow(x: ($T, u32)) -> $T {
                    let (self_val, n_val) = x;
                    self_val.saturating_pow(n_val)
                }
                let n = n.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), n),
                    _saturating_pow,
                )))
            }

            pub fn wrapping_add(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _wrapping_add(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_add(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_add,
                )))
            }

            pub fn wrapping_sub(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _wrapping_sub(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_sub(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_sub,
                )))
            }

            pub fn wrapping_mul(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _wrapping_mul(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_mul(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_mul,
                )))
            }

            pub fn wrapping_div(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _wrapping_div(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_div(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_div,
                )))
            }

            pub fn wrapping_div_euclid(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _wrapping_div_euclid(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_div_euclid(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_div_euclid,
                )))
            }

            pub fn wrapping_rem(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _wrapping_rem(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_rem(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_rem,
                )))
            }

            pub fn wrapping_rem_euclid(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _wrapping_rem_euclid(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_rem_euclid(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_rem_euclid,
                )))
            }

            pub fn wrapping_neg(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    $T::wrapping_neg,
                )))
            }

            pub fn wrapping_shl(&self, rhs: impl Into<$crate::SymU32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _wrapping_shl(x: ($T, u32)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_shl(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_shl,
                )))
            }

            pub fn wrapping_shr(&self, rhs: impl Into<$crate::SymU32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _wrapping_shr(x: ($T, u32)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.wrapping_shr(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _wrapping_shr,
                )))
            }

            pub fn wrapping_pow(&self, n: impl Into<$crate::SymU32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _wrapping_pow(x: ($T, u32)) -> $T {
                    let (self_val, n_val) = x;
                    self_val.wrapping_pow(n_val)
                }
                let n = n.into();
                $S::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), n),
                    _wrapping_pow,
                )))
            }

            pub fn pow(&self, exp: impl Into<$crate::SymU32<C, E>>) -> $S<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _pow(x: ($T, u32)) -> $T {
                    let (self_val, exp_val) = x;
                    self_val.pow(exp_val)
                }
                let exp = exp.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), exp), _pow)))
            }

            pub fn isqrt(&self) -> $S<C, E> {
                $S::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::isqrt)))
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

            pub fn ilog(&self, base: impl Into<$S<C, E>>) -> $crate::SymU32<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _ilog(x: ($T, $T)) -> u32 {
                    let (self_val, base_val) = x;
                    self_val.ilog(base_val)
                }
                let base = base.into();
                $crate::SymU32::Expr(E::lift($crate::SymFn::new((self.clone(), base), _ilog)))
            }

            pub fn ilog2(&self) -> $crate::SymU32<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                $crate::SymU32::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::ilog2)))
            }

            pub fn ilog10(&self) -> $crate::SymU32<C, E>
            where
                C: $crate::SymCtx<u32>,
                E: $crate::SymExpr<u32>,
            {
                $crate::SymU32::Expr(E::lift($crate::SymFn::new((self.clone(),), $T::ilog10)))
            }

            pub fn checked_ilog(&self, base: impl Into<$S<C, E>>) -> $crate::Sym<Option<u32>, C, E>
            where
                C: $crate::SymCtx<Option<u32>>,
                E: $crate::SymExpr<Option<u32>>,
            {
                #[inline(always)]
                fn _checked_ilog(x: ($T, $T)) -> Option<u32> {
                    let (self_val, base_val) = x;
                    self_val.checked_ilog(base_val)
                }
                let base = base.into();
                $crate::Sym::<Option<u32>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), base),
                    _checked_ilog,
                )))
            }

            pub fn checked_ilog2(&self) -> $crate::Sym<Option<u32>, C, E>
            where
                C: $crate::SymCtx<Option<u32>>,
                E: $crate::SymExpr<Option<u32>>,
            {
                #[inline(always)]
                fn _checked_ilog2(x: $T) -> Option<u32> {
                    x.checked_ilog2()
                }
                $crate::Sym::<Option<u32>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    _checked_ilog2,
                )))
            }

            pub fn checked_ilog10(&self) -> $crate::Sym<Option<u32>, C, E>
            where
                C: $crate::SymCtx<Option<u32>>,
                E: $crate::SymExpr<Option<u32>>,
            {
                #[inline(always)]
                fn _checked_ilog10(x: $T) -> Option<u32> {
                    x.checked_ilog10()
                }
                $crate::Sym::<Option<u32>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    _checked_ilog10,
                )))
            }

            pub fn checked_neg(&self) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>>,
                E: $crate::SymExpr<Option<$T>>,
            {
                #[inline(always)]
                fn _checked_neg(x: $T) -> Option<$T> {
                    x.checked_neg()
                }
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(),),
                    _checked_neg,
                )))
            }

            pub fn checked_shl(
                &self,
                rhs: impl Into<$crate::SymU32<C, E>>,
            ) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>> + $crate::SymCtx<u32>,
                E: $crate::SymExpr<Option<$T>> + $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _checked_shl(x: ($T, u32)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_shl(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_shl,
                )))
            }

            pub fn checked_shr(
                &self,
                rhs: impl Into<$crate::SymU32<C, E>>,
            ) -> $crate::Sym<Option<$T>, C, E>
            where
                C: $crate::SymCtx<Option<$T>> + $crate::SymCtx<u32>,
                E: $crate::SymExpr<Option<$T>> + $crate::SymExpr<u32>,
            {
                #[inline(always)]
                fn _checked_shr(x: ($T, u32)) -> Option<$T> {
                    let (self_val, rhs_val) = x;
                    self_val.checked_shr(rhs_val)
                }
                let rhs = rhs.into();
                $crate::Sym::<Option<$T>, C, E>::Expr(E::lift($crate::SymFn::new(
                    (self.clone(), rhs),
                    _checked_shr,
                )))
            }

            pub fn midpoint(&self, rhs: impl Into<$S<C, E>>) -> $S<C, E> {
                #[inline(always)]
                fn _midpoint(x: ($T, $T)) -> $T {
                    let (self_val, rhs_val) = x;
                    self_val.midpoint(rhs_val)
                }
                let rhs = rhs.into();
                $S::Expr(E::lift($crate::SymFn::new((self.clone(), rhs), _midpoint)))
            }
        }
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
