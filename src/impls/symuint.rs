use crate::{SymBool, SymCtx, SymExpr, uint_impl};

uint_impl!(u8, SymU8, i8, SymI8);
uint_impl!(u16, SymU16, i16, SymI16);
uint_impl!(u32, SymU32, i32, SymI32);
uint_impl!(u64, SymU64, i64, SymI64);
uint_impl!(u128, SymU128, i128, SymI128);
uint_impl!(usize, SymUsize, isize, SymIsize);

impl<C, E> SymU8<C, E>
where
    C: SymCtx<u8>,
    E: SymExpr<u8>,
{
    pub fn is_ascii(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii(x: u8) -> bool {
            x.is_ascii()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii",
            (self.clone(),),
            _is_ascii,
        )))
    }

    pub fn to_ascii_uppercase(&self) -> Self {
        fn _to_ascii_uppercase(x: u8) -> u8 {
            x.to_ascii_uppercase()
        }
        Self::Expr(E::lift(crate::SymFn::new(
            "to_ascii_uppercase",
            (self.clone(),),
            _to_ascii_uppercase,
        )))
    }

    pub fn to_ascii_lowercase(&self) -> Self {
        fn _to_ascii_lowercase(x: u8) -> u8 {
            x.to_ascii_lowercase()
        }
        Self::Expr(E::lift(crate::SymFn::new(
            "to_ascii_lowercase",
            (self.clone(),),
            _to_ascii_lowercase,
        )))
    }

    pub fn eq_ignore_ascii_case(&self, other: impl Into<Self>) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _eq_ignore_ascii_case(x: (u8, u8)) -> bool {
            let (self_val, other) = x;
            self_val.eq_ignore_ascii_case(&other)
        }
        let other = other.into();
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "eq_ignore_ascii_case",
            (self.clone(), other),
            _eq_ignore_ascii_case,
        )))
    }

    pub fn is_ascii_alphabetic(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_alphabetic(x: u8) -> bool {
            x.is_ascii_alphabetic()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_alphabetic",
            (self.clone(),),
            _is_ascii_alphabetic,
        )))
    }

    pub fn is_ascii_uppercase(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_uppercase(x: u8) -> bool {
            x.is_ascii_uppercase()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_uppercase",
            (self.clone(),),
            _is_ascii_uppercase,
        )))
    }

    pub fn is_ascii_lowercase(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_lowercase(x: u8) -> bool {
            x.is_ascii_lowercase()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_lowercase",
            (self.clone(),),
            _is_ascii_lowercase,
        )))
    }

    pub fn is_ascii_alphanumeric(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_alphanumeric(x: u8) -> bool {
            x.is_ascii_alphanumeric()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_alphanumeric",
            (self.clone(),),
            _is_ascii_alphanumeric,
        )))
    }

    pub fn is_ascii_digit(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_digit(x: u8) -> bool {
            x.is_ascii_digit()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_digit",
            (self.clone(),),
            _is_ascii_digit,
        )))
    }

    pub fn is_ascii_hexdigit(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_hexdigit(x: u8) -> bool {
            x.is_ascii_hexdigit()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_hexdigit",
            (self.clone(),),
            _is_ascii_hexdigit,
        )))
    }

    pub fn is_ascii_punctuation(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_punctuation(x: u8) -> bool {
            x.is_ascii_punctuation()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_punctuation",
            (self.clone(),),
            _is_ascii_punctuation,
        )))
    }

    pub fn is_ascii_graphic(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_graphic(x: u8) -> bool {
            x.is_ascii_graphic()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_graphic",
            (self.clone(),),
            _is_ascii_graphic,
        )))
    }

    pub fn is_ascii_whitespace(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_whitespace(x: u8) -> bool {
            x.is_ascii_whitespace()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_whitespace",
            (self.clone(),),
            _is_ascii_whitespace,
        )))
    }

    pub fn is_ascii_control(&self) -> SymBool<C, E>
    where
        C: SymCtx<bool>,
        E: SymExpr<bool>,
    {
        fn _is_ascii_control(x: u8) -> bool {
            x.is_ascii_control()
        }
        SymBool::<C, E>::Expr(E::lift(crate::SymFn::new(
            "is_ascii_control",
            (self.clone(),),
            _is_ascii_control,
        )))
    }
}
