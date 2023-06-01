use core::ffi::CStr;
use cstr_literal::cstr;

const fn s() -> &'static CStr {
  cstr!("test")
}

fn main() {
  assert_eq!(s().to_bytes_with_nul(), b"test\0");
}
