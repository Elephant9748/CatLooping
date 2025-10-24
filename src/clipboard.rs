use std::{
    env,
    process::{Command, Stdio},
    thread,
};

use colored::Colorize;

pub fn copy_clipboard(text: &str) {
    if env::var("XDG_SESSION_TYPE").unwrap() == "wayland" {
        let wl_copy_check = Command::new("sh")
            .args(["-c", "command -v wl-copy"])
            .stdout(Stdio::piped())
            .output()
            .unwrap_or_else(|_| {
                panic!(
                    "{}{}",
                    ">".bright_yellow(),
                    " doesnt have wl-clipboard".bright_red()
                )
            });
        if wl_copy_check.stdout.is_empty() {
            println!(
                "{}{}",
                ">".bright_yellow(),
                " doesnt have wl-clipboard".white()
            )
        } else {
            Command::new("wl-copy")
                .args([text])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap_or_else(|_| {
                    panic!(
                        "{}{}",
                        ">".bright_yellow(),
                        " wl-copy copy failed.".bright_red()
                    )
                })
                .wait()
                .expect("--> Failed to wait copy_clipboard()");
        }
    } else {
        println!(
            "{}{}",
            ">".bright_yellow(),
            " Not in wayland session".bright_red()
        );
    }
    clear_clipboard();
}
pub fn clear_clipboard() {
    let clear_clipboard_duration = 30; //default 30s until clipboard clear
    let thread_clear_clipboard = thread::spawn(move || {
        Command::new("sh")
            .args([
                "-c",
                format!("sleep {} && wl-copy -c", clear_clipboard_duration).as_str(),
            ])
            .stdout(Stdio::piped())
            .spawn()
            .expect("--> Thread failed No bash found clear_clipboard()")
            .try_wait()
            .expect("--> Failed to try_wait clear_clipboard()");
    });

    if thread_clear_clipboard.join().is_ok() {
        println!(
            "{}{}{}",
            ">".bright_yellow(),
            " Clear clipboard  after".bright_yellow(),
            format!(" {} sec", clear_clipboard_duration).bright_green()
        );
    }
}
