# SYMEXPR

![Rust](https://github.com/cksac/symexpr/workflows/Rust/badge.svg)
[![Docs Status](https://docs.rs/symexpr/badge.svg)](https://docs.rs/symexpr)
[![Latest Version](https://img.shields.io/crates/v/symexpr.svg)](https://crates.io/crates/symexpr)

`symexpr` is a Rust library for symbolic expressions and evaluation, supporting Rust primitive types and most of their functions. Easily extend symbolic operations and turning custom type to symbolic type.

---

## Features
- Symbolic representation and evaluation for Rust primitive types (integers, floats, bools, char, etc.)
- Supports most functions and operators for Rust primitive types symbolic representation
- Easy to turn regular function in value type as experssion for sybmolic representation
- Easy to extend to custom type as symbolic representation Sym<T, C, E>
- Support custom context type during evaluation
- Support Symoblic experssion stored as Rc / Arc or other pointer type


## Installation
```sh
cargo add symexpr
```

## Usage
```rust
use symexpr::{Context, SymCtx, SymValue, SymUsize, SymF32};
type Usize = SymUsize;
type F32 = SymF32;

fn main() {
    // Symbolic variables and constants
    let x = &Usize::symbol("a");
    let y = &Usize::constant(4);
    let k = &Usize::constant(2);

    // Context for evaluation
    let mut ctx = Context::default();
    ctx.set_symbol("a", 2usize);

    // Evaluate symbolic values
    assert_eq!(x.eval(&ctx).unwrap(), 2);
    assert_eq!(y.eval(&ctx).unwrap(), 4);

    // Symbolic comparisons
    let b = x.eq(y);
    assert!(!b.eval(&ctx).unwrap());
    let b = x.ge(y);
    assert!(!b.eval(&ctx).unwrap());

    // Symbolic arithmetic
    let z = x + y;
    assert_eq!(z.eval(&ctx).unwrap(), 6);

    // Mixed expressions
    let c = 3;
    let w = c + z + 2 + k + 3;
    assert_eq!(w.eval(&ctx).unwrap(), 16);

    // Symbolic math functions (e.g., floor)
    let xf = F32::symbol("f");
    ctx.set_symbol("f", 3.7f32);
    let floor_xf = xf.floor();
    assert_eq!(floor_xf.eval(&ctx).unwrap(), 3.0);
}
```

## Extending symbolic operations

You can extend symbolic operations in two main ways:

### 1. Implement symbolic functions using `SymFn`

For most functions (e.g., `abs`, `floor`, `sqrt`), you can wrap the function using `SymFn`:

```rust
use symexpr::{Sym, SymCtx, SymFn, SymF32};

impl<C, E> SymF32<C, E>
where
    C: SymCtx<f32>,
    E: SymExpr<f32>,
{
    pub fn abs(&self) -> SymF32<C, E> {
        SymF32::<C, E>::Expr(E::lift(SymFn::new("abs", (self.clone(),), f32::abs)))
    }

    pub fn max(&self, other: impl Into<SymF32<C, E>>) -> SymF32<C, E> {
        #[inline(always)]
        fn _max(x: (f32, f32)) -> f32 {
            let (self_val, other_val) = x;
            self_val.max(other_val)
        }
        let other = other.into();
        SymF32::<C, E>::Expr(E::lift(SymFn::new(
            "max",
            (self.clone(), other),
            _max,
        )))
    }
}

let x = SymF32::symbol("x");
let abs_x = x.abs();
```

### 2. Implement a trait with an Op struct

For custom or more complex operations, define an Op struct and implement the corresponding trait (e.g., for symbolic equality):

```rust
use crate::{Result, Sym, SymCtx, SymExpr, SymValue, Value};

pub trait SymEq<C, E, R>
where
    C: SymCtx<bool>,
    E: SymExpr<bool>,
{
    fn eq(self, rhs: R) -> Sym<bool, C, E>;
}

impl<C, E, L, R> SymValue<C> for Eq<C, E, L, R>
where
    L: Value,
    R: Value,
    C: SymCtx<L> + SymCtx<R>,
    E: SymExpr<L> + SymExpr<R>,
    L: std::cmp::PartialEq<R>,
{
    type Value = bool;

    fn eval(&self, ctx: &C) -> Result<Self::Value> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        Ok(lhs == rhs)
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.lhs.display(f)?;
        f.write_str(" == ")?;
        self.rhs.display(f)?;
        f.write_str(")")
    }
}

// Then implement a trait to use the Op struct
pub trait SymEq<C, E, R> {
    fn eq(self, rhs: R) -> Sym<bool, C, E>;
}

impl<C, E, LHS, RHS> SymEq<C, E, Sym<RHS, C, E>> for Sym<LHS, C, E>
where
    C: SymCtx<LHS> + SymCtx<RHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<RHS> + SymExpr<bool>,
    LHS: Value + std::cmp::PartialEq<RHS>,
    LHS: Value,
    RHS: Value,
    bool: Value,
{
    fn eq(self, rhs: Sym<RHS, C, E>) -> Sym<bool, C, E> {
        Sym::<bool, C, E>::Expr(E::lift(Eq::new(self, rhs)))
    }
}

impl<C, E, LHS> Sym<LHS, C, E>
where
    LHS: Value,
    C: SymCtx<LHS> + SymCtx<bool>,
    E: SymExpr<LHS> + SymExpr<bool>,
{
    #[inline(always)]
    pub fn eq<RHS>(&self, rhs: RHS) -> Sym<bool, C, E>
    where
        for<'a> &'a Self: SymEq<C, E, RHS>,
    {
        SymEq::eq(self, rhs)
    }
}
```
