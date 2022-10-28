use paper_backup::lib::*;
use std::env;
use std::process::{Command, Stdio};
use colored::Colorize;

fn main() {

    let mut main_args: Vec<String> = env::args().collect();
    let mut not_in_the_menu = String::new();
    
    main_args.remove(0);

    for arg in main_args {

        if arg == "--help" {
            menu_option(Menu::Help);
        }

        if arg == "--diceware" {

            print!("{}", "\n> Home many wordlist: ".bright_green());
            let helper_catch_stdin = catch_stdin();

            menu_option(Menu::Diceware(helper_catch_stdin)); 
        }

        if arg == "--eff" {
            menu_option(Menu::Eff);
        }

        if arg == "--lock" {
            print!("{}", "Store string: ".bright_green());
            let y = catch_stdin();
            store_passphrase(y);

            // print!("\n{}", "Enter string to ecnrypt: ".bright_yellow());
            // let x = catch_stdin();
            // gpg_encrypt(x);
        }

        if arg != "--eff" && arg != "--diceware" && arg != "--help" && arg != "--lock" {
                not_in_the_menu.push_str(&arg);
                not_in_the_menu.push(' ');
        }
    }

    if not_in_the_menu != "" {
        let mut value = not_in_the_menu.clone();
        value.pop();

        menu_option(Menu::Notenum(value));
    }

}

fn _gpg_encrypt(_val: String){ 
    let gpg = Command::new("gpg")
        .args(&[
              "-o","secret.gpg","--symmetric","--s2k-mode","3","--s2k-count","65011712","--s2k-digest-algo",
              "SHA512","--cipher-algo","AES256","--armor", "frost"
        ])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute diceware");

    let gpg_utf8 = String::from_utf8_lossy(&gpg.stdout);
    let gpg_utf8_err = String::from_utf8_lossy(&gpg.stderr);

    println!("{}\n{:?}\n{}\n{}","Gpg encrypt result: ".yellow(), gpg, gpg_utf8, gpg_utf8_err);

    // let gpg_utf8_split = gpg_utf8.split("\n");

    // let gpg_vec: Vec<&str> = gpg_utf8_split.collect();

    // gpg_vec.into_iter().filter(|v| v.to_string() != "").map(|x| x.to_string()).collect();

}

fn store_passphrase(_val: String) {
}



