use assert_cmd::Command;
use assert_cmd::cargo; // マクロはこのモジュール配下
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    // マクロは PathBuf を返す
    let bin = cargo::cargo_bin!("echor");
    let mut cmd = Command::new(bin);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}
