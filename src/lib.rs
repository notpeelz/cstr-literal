#![doc = include_str!("../README.md")]
#![deny(unsafe_op_in_unsafe_fn)]
#![no_std]

#[doc(hidden)]
pub mod private {
  use core::ffi::CStr;

  pub extern crate const_format;

  pub const fn new(s: &'static str) -> &'static CStr {
    let mut bytes = s.as_bytes();
    loop {
      match bytes {
        [0, _, ..] => panic!("C strings can't contain null bytes"),
        [] => panic!("C strings must be null-terminated"),
        [0] => break,
        [_, remaining @ ..] => bytes = remaining,
      }
    }

    unsafe { CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
  }
}

/// Creates a [`&'static CStr`][`core::ffi::CStr`] from a const [`&'static str`][`str`].
///
/// The input string can be a literal or a const expression.
///
/// Under the hood, this macro uses [`const_format::concatcp`] to append
/// the null terminator and therefore is subject to the same limitations.
#[macro_export]
macro_rules! cstr {
  ($s:expr) => {{
    const __CSTR_STR: &str = $crate::private::const_format::concatcp!($s, "\0");
    const __CSTR: &::core::ffi::CStr = $crate::private::new(__CSTR_STR);
    __CSTR
  }};
}

#[cfg(test)]
mod tests {
  #[test]
  fn valid() {
    let t = trybuild::TestCases::new();
    t.pass("tests/valid/*.rs");
  }

  #[test]
  fn invalid() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/invalid/*.rs");
  }
}
