use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

static EXPECTED_OUTPUT: &'static str = "नमस्ते, world!";

#[test]
fn call_program() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::main_binary()?;
  cmd
    .assert()
    .stdout(predicate::str::contains(EXPECTED_OUTPUT));
  Ok(())
}
