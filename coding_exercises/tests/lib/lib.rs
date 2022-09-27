#[cfg(test)]
pub mod lib_test{
  #[test]
  fn hello_lib_test() {
    use cargo_challenges_lib as lib;
    lib::hello_challenges();
    
  }
}
