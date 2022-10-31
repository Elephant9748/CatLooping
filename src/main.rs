use paper_backup::lib::*;
use colored::Colorize;
use std::env;

fn main() {

    let mut main_args: Vec<String> = env::args().collect();
    let mut not_in_the_menu = String::new();
    
    main_args.remove(0);

    for arg in main_args {

        if arg == "--help" {
            menu_option(Menu::Help);
        }

        if arg == "--diceware-lock" {
            print!("{}", "\n> Home many wordlist: ".bright_green());
            let helper_catch_stdin = catch_stdin();

            menu_option(Menu::Diceware(helper_catch_stdin)); 
        }

        if arg == "--eff" {
            menu_option(Menu::Eff);
        }

        if arg != "--eff" && arg != "--diceware-lock" && arg != "--help" && arg != "--lock" {
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





