# cstr-literal

[![crates.io](https://img.shields.io/crates/v/cstr-literal.svg?color=green&style=for-the-badge&logo=rust)](https://crates.io/crates/cstr-literal)
[![license](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)

This crate provides `cstr!`, a const-friendly macro for C string literals.

_Compiler support: requires rustc 1.64+_

## Why?

Rust doesn't have C string literals ([yet](https://rust-lang.github.io/rfcs/3348-c-str-literal.html)).

As of writing this, there's a couple `cstr!` macros floating around, but they
all have their own set of drawbacks (unmaintained, no const support, nightly-only, overly complex/buggy, etc.)

## Examples

### Simple literal

```rust
use core::ffi::CStr;
use cstr_literal::cstr;

const STR: &CStr = cstr!("test");

fn test() {
  assert_eq!(STR.to_bytes_with_nul(), b"test\0");
}
```

### With references to other const items

```rust
use core::ffi::{c_char, CStr};
use cstr_literal::cstr;

const ALLOCATOR: &str = "malloc";

extern "C" {
  fn use_allocator(name: *const c_char);
}

fn test() {
  unsafe { use_allocator(cstr!(ALLOCATOR).as_ptr()) };
}
```

### With [const_format](https://crates.io/crates/const_format)

```rust
use core::ffi::CStr;
use cstr_literal::cstr;
use const_format::formatcp;

const VERSION: &CStr = {
  const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
  const GIT_HEAD: &str = "47007ba";
  cstr!(formatcp!("{PKG_VERSION}+{GIT_HEAD}"))
};

fn test() {
  assert_eq!(VERSION.to_bytes_with_nul(), b"0.1.0+47007ba\0");
}
```

## No-std support

Thanks to [rust#94079](https://github.com/rust-lang/rust/pull/94079), this crate is unconditionally `#![no_std]`.
