use core::ffi::CStr;
use cstr_literal::cstr;

const fn s() -> &'static CStr {
  const STR: &str = "foo";
  cstr!(STR)
}

fn main() {
  assert_eq!(s().to_bytes_with_nul(), b"foo\0");
}
