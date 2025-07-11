# SYMVAL

![Rust](https://github.com/cksac/symval/workflows/Rust/badge.svg)
[![Docs Status](https://docs.rs/symval/badge.svg)](https://docs.rs/symval)
[![Latest Version](https://img.shields.io/crates/v/symval.svg)](https://crates.io/crates/symval)

`symval` is a Rust libray to build symbolic expression and evaluation.
---

## Installation
```sh
cargo add symval
```

## Usage
```rust
use symval::ops::{SymEq, SymGe};
use symval::{Context, SymCtx, SymValue, SymUsize};

type Usize = SymUsize;

#[test]
fn it_works() {
    let x = &Usize::symbol("a");
    let y = &Usize::constant(4);
    let k = &Usize::constant(2);

    let mut ctx = Context::default();
    ctx.set_symbol("a", 2usize);

    let result = x.eval(&ctx).unwrap();
    assert_eq!(result, 2);

    let result = y.eval(&ctx).unwrap();
    assert_eq!(result, 4);

    let b = x.eq(y);
    let result = b.eval(&ctx).unwrap();
    assert_eq!(result, false);

    let b = x.ge(y);
    let result = b.eval(&ctx).unwrap();
    assert_eq!(result, false);

    let z = x + y;
    let result = z.eval(&ctx).unwrap();
    assert_eq!(result, 6);

    let c = 3;
    let w = c + z + 2 + k + 3;
    let result = w.eval(&ctx).unwrap();
    assert_eq!(result, 16);

    println!("{:?}", w);
}
```