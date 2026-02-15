use assert_cmd::Command;
use assert_cmd::cargo;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    // bin = /Users/niiyama/ws/rust_playground/echor/target/debug/echor
    // println!("bin: {}", bin.display()); で確認できる
    let bin = cargo::cargo_bin!("echor");
    let mut cmd = Command::new(bin);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let echor_cmd = cargo::cargo_bin!("echor");
    let echor_out = Command::new(echor_cmd)
        .arg("-n")
        .arg("hello")
        .output()
        .expect("failed to run echor");
    let echo_out = Command::new("bash")
        .arg("-c")
        .arg("echo -n hello")
        .output()
        .expect("failed to run echo");

    // assert!(echor_out.status.success(), "echor exited with failure");
    assert!(echo_out.status.success(), "echo exited with failure");

    assert_eq!(echor_out.stdout, echo_out.stdout, "stdout mismatch");
}
