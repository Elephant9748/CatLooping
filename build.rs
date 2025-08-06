use chrono::{TimeZone, Utc};
use std::{
    env,
    process::{Command, Stdio},
};

fn main() {
    let git_hash = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .stdout(Stdio::piped())
        .output()
        .expect("Get git short hash failed");
    let build_date = match env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => Utc
            .timestamp_opt(val.parse::<i64>().unwrap(), 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
        Err(_) => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    println!("cargo:rustc-env=DATE={}", build_date);
    println!(
        "cargo:rustc-env=GIT_HASH={}",
        str::from_utf8(&git_hash.stdout).unwrap(),
    );
}
