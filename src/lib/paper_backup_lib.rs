pub mod lib {

use std::process::{Command, Stdio};
use colored::Colorize;
use std::{io, io::Write};

    pub enum Menu {
        Help,
        Eff,
        Diceware(String),
        Notenum(String),
    }

    pub fn get_help() {
        println!("\nrequire: ");
        println!("       - rust-diceware binary from crate.io manually installed");
        println!("");
        println!("usage: paper_backup [--help] [--eff]");
        println!("");
        println!("option: ");
        println!("       --help      :  Help command!");
        println!("       --eff       :  Generate Eff random wordlist");
        println!("       --diceware  :  Generate passphrase using diceware crate\n");
    }

    pub fn menu_option(menu_list: Menu) {
        match menu_list {
            Menu::Help =>  get_help(),
            Menu::Diceware(arg) => {
                println!("{}", "> diceware".bright_cyan());
                println!("{}", "---------".bright_cyan());
                println!("{}{}", "entropy   : ".cyan(), diceware_generate(arg.as_str(),"minilock","-")[1]);
                println!("{}{}\n", "passphrase: ".green(), diceware_generate(arg.as_str(),"minilock","-")[0]
                         .color("white")
                         .on_color("black")
                         .italic()
                );
            },
            Menu::Eff => {
                println!("\neff wordlist");
                println!("------------");
                println!("{}{}\n","Output: ".green(),  generate_eff_word());
            }
            Menu::Notenum(arg) => {
                println!("> {} {}",arg.bright_red(), "Menu Argument not available please check help: --help".bright_yellow());
            }
        }
    }

    pub fn diceware_generate(n_value: &str, wordlist: &str, delimiter: &str) -> Vec<String> {
        let diceware = Command::new("diceware")
            .args(&["-d", delimiter,"-e","-n", n_value,"-l",wordlist])
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute diceware");

        let dice = String::from_utf8_lossy(&diceware.stdout);

        let dice_split = dice.split("\n");

        let dice_vec: Vec<&str> = dice_split.collect();

        dice_vec.into_iter().filter(|v| v.to_string() != "").map(|x| x.to_string()).collect()
    }

    // generate from eff-wordlist crates
    pub fn generate_eff_word() -> String {
        let mut words: Vec<String> = Vec::new();

        let mut words_string = String::new();

        while words.len() < 20 {
            let word = eff_wordlist::large::random_word();
            words.push(word.to_string());
            words_string.push_str(&word.to_string());
            words_string.push('-');
        }

        words_string.pop();


        words_string
    }
    
    pub fn catch_stdin() -> String {
        let mut input = String::new();

        let _ = io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Something wrong with stdin().read_line() !");

        input.trim().to_string()
    }
}
