use paper_backup::lib::*;
use colored::Colorize;
use std::env;

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
            },
            val if val == "--diceware-lock" => {
                print!("{}", "\n> How many wordlist: ".bright_green());
                let helper_catch_stdin = catch_stdin();

                menu_option(Menu::DicewareLock(helper_catch_stdin)); 
            },
            val if val == "--eff" => {
                print!("\n{}", "> Count word : ".bright_green());
                let count_word = catch_stdin();
                menu_option(Menu::Eff(count_word.parse().unwrap()));
            },
            val if val == "--eff-lock" => {
                print!("\n{}", "> Count word : ".bright_green());
                let count_word = catch_stdin();
                menu_option(Menu::EffLock(count_word.parse().unwrap()))
            },
            val if val == "--unlock" => menu_option(Menu::Unlock),
            val if val == "--convert" => menu_option(Menu::Convert),
            val if val != "--eff" && val != "--diceware-lock" && val != "--help" 
                && val != "--diceware" && val != "--unlock" && val != "--convert"
                && val != "eff-lock" => {
                    not_in_the_menu.push_str(&val);
                    not_in_the_menu.push(' ');
                },
            _ => println!("{}", "> match arg main not found list".bright_red()),
        }

    }

    if not_in_the_menu != "" {
        let mut value = not_in_the_menu.clone();
        value.pop();
        menu_option(Menu::Notenum(value));
    }

}





