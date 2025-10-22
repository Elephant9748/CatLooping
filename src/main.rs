use colored::Colorize;
// use paper_backup::lib::*;
use std::env;

mod cipher_string;
mod clipboard;
mod config;
mod entropy_check;
mod env_paper;
mod paper_backup;
mod pass_gen;
mod steg;

use paper_backup::{catch_stdin, menu_option, mnemonic_menu_list, Menu};

fn main() {
    let mut main_args: Vec<String> = env::args().collect();
    main_args.remove(0);

    let mut check_option_double = false;
    match main_args.len() {
        1 => {
            for m in main_args.to_owned() {
                match m {
                    arg if arg == "--help" || arg == "-h" => menu_option(Menu::Help),
                    arg if arg == "--version" || arg == "-v" => menu_option(Menu::Version),
                    arg if arg == "--diceware" => {
                        print!("{}", "\n> How many wordlist: ".bright_green());
                        let diceware_only = catch_stdin();

                        menu_option(Menu::Diceware(diceware_only));
                    }
                    arg if arg == "--diceware-lock" => {
                        print!("{}", "\n> How many wordlist: ".bright_green());
                        let helper_catch_stdin = catch_stdin();

                        menu_option(Menu::DicewareLock(helper_catch_stdin));
                    }
                    arg if arg == "--eff" => {
                        print!("\n{}", "> Count word : ".bright_green());
                        let count_word = catch_stdin();
                        menu_option(Menu::Eff(count_word.parse().unwrap()));
                    }
                    arg if arg == "--eff-lock" => {
                        print!("\n{}", "> Count word : ".bright_green());
                        let count_word = catch_stdin();
                        menu_option(Menu::EffLock(count_word.parse().unwrap()))
                    }
                    arg if arg == "--mnemonic" => {
                        let menu = mnemonic_menu_list();
                        menu_option(Menu::MnemonicGen(
                            menu[0].trim().parse().unwrap(),
                            menu[1].to_string(),
                        ));
                    }
                    arg if arg == "--mnemonic-lock" => {
                        let menu = mnemonic_menu_list();
                        menu_option(Menu::MnemonicGenLock(
                            menu[0].trim().parse().unwrap(),
                            menu[1].to_string(),
                        ));
                    }
                    arg if arg == "--lock-string" => {
                        print!("\n{}", "> Type word wants to encrypt: ".bright_green());
                        let input_string_type = catch_stdin();
                        menu_option(Menu::LockString(input_string_type));
                    }
                    arg if arg == "--from-file" => {
                        print!("\n{}", "> Path of file: ".bright_green());
                        let path = catch_stdin();
                        menu_option(Menu::FromFile(path));
                    }
                    arg if arg == "--from-file-pgp" => {
                        print!("\n{}", "> Path of file: ".bright_green());
                        let path = catch_stdin();
                        menu_option(Menu::FromFileLock(path));
                    }
                    arg if arg == "--qrcode-no-pgp" => {
                        print!("\n{}", "> Input string: ".bright_yellow());
                        let raw_string = catch_stdin();

                        menu_option(Menu::QrOnly(raw_string));
                    }
                    arg if arg == "--entropy-check" => {
                        print!("\n{}", "> check entropy: ".bright_yellow());
                        let raw_string = catch_stdin();

                        menu_option(Menu::Entropy(raw_string));
                    }
                    arg if arg == "--password" => {
                        print!("\n{}", "> password lenght: ".bright_red());
                        let raw_string = catch_stdin();

                        menu_option(Menu::GenPassword(raw_string));
                    }
                    arg if arg == "--encode-image" => {
                        print!("\n{}", ":: > Hide messages: ".bright_red());
                        let raw_string = catch_stdin();

                        menu_option(Menu::EncodeImage(raw_string));
                    }
                    arg if arg == "--decode-image" => {
                        menu_option(Menu::DecodeImage);
                    }
                    arg if arg == "--set-config" => {
                        menu_option(Menu::Config);
                    }
                    arg if arg == "--config" => {
                        menu_option(Menu::ShowConfig);
                    }
                    arg if arg == "--unlock" => menu_option(Menu::Unlock),
                    arg if arg == "--convert" => menu_option(Menu::Convert),
                    _ => {
                        check_option_double = true;
                    }
                }
            }
        }
        _ => check_option_double = true,
    }

    if check_option_double {
        let mut not_menu = String::new();
        for val in main_args {
            not_menu.push_str(val.as_str());
            not_menu.push(' ');
        }
        menu_option(Menu::Notenum(not_menu));
    }
}
