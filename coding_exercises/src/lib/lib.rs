// https://users.rust-lang.org/t/should-unit-tests-really-be-put-in-the-same-file-as-the-source/62153

pub mod challenges;
pub mod constants;

// Test
pub fn hello_lib() {
  challenges::hello_challenges();
  print!("{}", constants::MARK_ALLEN_JAVA_CHALLENGES_PATH);
}

