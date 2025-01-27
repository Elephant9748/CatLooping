use colored::Colorize;
// use paper_backup::lib::*;
use std::env;

mod cipher_string;
mod entropy_check;
mod paper_backup;
mod pass_gen;

use crate::paper_backup::*;

fn main() {
    let mut main_args: Vec<String> = env::args().collect();
    let mut not_in_the_menu = String::new();

    main_args.remove(0);

    for arg in main_args {
        match arg {
            val if val == "--help" => menu_option(Menu::Help),
            val if val == "--diceware" => {
                print!("{}", "\n> How many wordlist: ".bright_green());
                let diceware_only = catch_stdin();

                menu_option(Menu::Diceware(diceware_only));
            }
            val if val == "--diceware-lock" => {
                print!("{}", "\n> How many wordlist: ".bright_green());
                let helper_catch_stdin = catch_stdin();

                menu_option(Menu::DicewareLock(helper_catch_stdin));
            }
            val if val == "--eff" => {
                print!("\n{}", "> Count word : ".bright_green());
                let count_word = catch_stdin();
                menu_option(Menu::Eff(count_word.parse().unwrap()));
            }
            val if val == "--eff-lock" => {
                print!("\n{}", "> Count word : ".bright_green());
                let count_word = catch_stdin();
                menu_option(Menu::EffLock(count_word.parse().unwrap()))
            }
            val if val == "--mnemonic" => {
                let menu = mnemonic_menu_list();
                menu_option(Menu::MnemonicGen(
                    menu[0].trim().parse().unwrap(),
                    menu[1].to_string(),
                ));
            }
            val if val == "--mnemonic-lock" => {
                let menu = mnemonic_menu_list();
                menu_option(Menu::MnemonicGenLock(
                    menu[0].trim().parse().unwrap(),
                    menu[1].to_string(),
                ));
            }
            val if val == "--lock-string" => {
                print!("\n{}", "> Type word wants to encrypt: ".bright_green());
                let input_string_type = catch_stdin();
                menu_option(Menu::LockString(input_string_type));
            }
            val if val == "--to-file" => {
                print!("\n{}", "> Path of file: ".bright_green());
                let path = catch_stdin();
                menu_option(Menu::ToFile(path));
            }
            val if val == "--qrcode-no-pgp" => {
                print!("\n{}", "> Input string: ".bright_yellow());
                let raw_string = catch_stdin();

                menu_option(Menu::QrOnly(raw_string));
            }
            val if val == "--entropy-check" => {
                print!("\n{}", "> check entropy: ".bright_yellow());
                let raw_string = catch_stdin();

                menu_option(Menu::Entropy(raw_string));
            }
            val if val == "--password" => {
                print!("\n{}", "> password lenght: ".bright_red());
                let raw_string = catch_stdin();

                menu_option(Menu::GenPassword(raw_string));
            }
            val if val == "--try" => {
                print!("--try menu option.");
            }
            val if val == "--unlock" => menu_option(Menu::Unlock),
            val if val == "--convert" => menu_option(Menu::Convert),
            val if val != "--eff"
                && val != "--diceware-lock"
                && val != "--help"
                && val != "--diceware"
                && val != "--unlock"
                && val != "--convert"
                && val != "--eff-lock"
                && val != "--try"
                && val != "--mnemonic"
                && val != "--mnemonic-lock"
                && val != "--lock-string"
                && val != "--entropy-check"
                && val != "--password" =>
            {
                not_in_the_menu.push_str(&val);
                not_in_the_menu.push(' ');
            }
            _ => println!("{}", "> match arg main not found list".bright_red()),
        }
    }

    if not_in_the_menu != "" {
        let mut value = not_in_the_menu.clone();
        value.pop();
        menu_option(Menu::Notenum(value));
    }
}
