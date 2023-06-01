use core::ffi::CStr;
use cstr_literal::cstr;

const fn s() -> &'static CStr {
  const STR: &str = "bar";
  cstr!(const_format::formatcp!("Foo: {}", STR))
}

fn main() {
  assert_eq!(s().to_bytes_with_nul(), b"Foo: bar\0");
}
