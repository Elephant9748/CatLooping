use std::{
    env,
    process::{Command, Stdio},
    thread,
};

use colored::Colorize;

pub fn copy_clipboard(text: &str) {
    if env::var("XDG_SESSION_TYPE").unwrap() == "wayland" {
        let wl_copy_check = Command::new("sh")
            .args(&["-c", format!("command -v wl-copy").as_str()])
            .stdout(Stdio::piped())
            .output()
            .expect(
                format!(
                    "{}{}",
                    ">".bright_yellow(),
                    " doesnt have wl-clipboard".bright_red()
                )
                .as_str(),
            );
        if wl_copy_check.stdout.is_empty() {
            println!("{}{}", ">".bright_yellow(), " doesnt have wl-clipboard")
        } else {
            Command::new("wl-copy")
                .args(&[text])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect(
                    format!(
                        "{}{}",
                        ">".bright_yellow(),
                        " wl-copy copy failed.".bright_red()
                    )
                    .as_str(),
                );
        }
    } else {
        println!(
            "{}{}",
            ">".bright_yellow(),
            " Not in wayland session".bright_red()
        );
    }
    // clear clipboard default 30s
    clear_clipboard();
}
pub fn clear_clipboard() {
    let clear_clipboard_duration = 30; //default 30s until clipboard clear
    let thread_clear_clipboard = thread::spawn(move || {
        Command::new("sh")
            .args(&[
                "-c",
                format!("sleep {} && wl-copy -c", clear_clipboard_duration).as_str(),
            ])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Thread failed No bash found.");
    });

    if thread_clear_clipboard.join().is_ok() {
        print!(
            "{}{}{}",
            ">".bright_yellow(),
            " Clear clipboard  after".bright_yellow(),
            format!(" {} sec", clear_clipboard_duration).bright_green()
        );
    }
}
