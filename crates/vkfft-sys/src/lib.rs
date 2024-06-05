#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
  #[test]
  fn version() {
    assert_eq!(unsafe { super::vkfft_get_version() }, 10304);
  }
}
