use std::process::{Command, Stdio};

use crate::cli::YEAR;

pub fn solve(day: u8, release: bool, test: bool) -> Result<(), String> {
    let cmd = if test { "test" } else { "run" };
    let bin_name = format!("day{:0>2}-{}", day, YEAR);
    let mut cmd_args = vec![cmd.to_string(), "--bin".to_string(), bin_name];

    if release {
        cmd_args.push("--release".to_string());
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
    Ok(())
}
