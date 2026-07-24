use chrono::{TimeZone, Utc};
use std::{
    env, fs,
    io::{self, BufRead},
    process::{Command, Stdio},
};

fn get_linux_distro() -> io::Result<Option<String>> {
    let file = fs::File::open("/etc/os-release")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("ID=")
            && let Some(value) = line.split('=').nth(1)
        {
            return Ok(Some(value.trim_matches('"').to_string()));
        }
    }
    Ok(None)
}

fn main() {
    if let Ok(Some(distro)) = get_linux_distro() {
        if distro == "nixos" {
            let git_hash = env::var("GIT_HASH").unwrap_or_else(|_| "unknown".to_string());

            let build_date = env::var("SOURCE_DATE_EPOCH")
                .map(|ts| {
                    let epoch = ts.parse::<i64>().unwrap_or(0);
                    chrono::Utc
                        .timestamp_opt(epoch, 0)
                        .unwrap()
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                })
                .unwrap_or_else(|_| "unknown".to_string());
            println!("cargo:rustc-env=DATE={}", build_date);
            println!("cargo:rustc-env=GIT_HASH={}", git_hash,);
            println!("cargo:rerun-if-env-changed=GIT_HASH");
            println!("cargo:rerun-if-env-changed=DATE");
        } else {
            let git_hash = Command::new("git")
                .args(["rev-parse", "--short", "HEAD"])
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
            println!("cargo:rerun-if-env-changed=GIT_HASH");
            println!("cargo:rerun-if-env-changed=DATE");
        }
    }
}
