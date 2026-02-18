use colored::Colorize;
use std::{
    env::var_os,
    path::Path,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct Dice<'dice> {
    n_value: Option<u32>,
    wordlist: Option<&'dice str>,
    delimiter: Option<&'dice str>,
}

impl<'dice> Dice<'dice> {
    pub fn new(n_value: u32, wordlist: &'dice str, delimiter: &'dice str) -> Self {
        Self {
            n_value: Some(n_value),
            wordlist: Some(wordlist),
            delimiter: Some(delimiter),
        }
    }
}

pub trait Diceware {
    fn generate_wordlist(&self) -> Vec<String>;
}

pub trait DiceVec {
    fn find_entropy(&self) -> Option<&str>;
    fn find_passphrase(&self) -> Option<&str>;
}

impl DiceVec for Vec<String> {
    fn find_entropy(&self) -> Option<&str> {
        if self.is_empty() {
            None
        } else {
            Some(&self[1])
        }
    }
    fn find_passphrase(&self) -> Option<&str> {
        if self.is_empty() {
            None
        } else {
            Some(&self[0])
        }
    }
}

impl<'dice> Diceware for Dice<'dice> {
    fn generate_wordlist(&self) -> Vec<String> {
        let path_diceware = format!(
            "{}/.cargo/bin/diceware",
            var_os("HOME").unwrap().to_owned().to_string_lossy()
        );
        let check_path_magick = Path::new(&path_diceware);
        let mut run_bin = Box::new("diceware");
        println!("{:?}", check_path_magick.exists());
        if check_path_magick.exists() {
            *run_bin = "diceware";
        } else {
            panic!("{}", "> diceware doesnt exists ..!".bright_red());
        }

        let diceware = Command::new(*run_bin)
            .args([
                "-d",
                self.delimiter.unwrap(),
                "-e",
                "-n",
                self.n_value.unwrap().to_string().as_str(),
                "-l",
                self.wordlist.unwrap(),
            ])
            .stdout(Stdio::piped())
            .output()
            .unwrap_or_else(|_| {
                panic!(
                    "{}{}",
                    "> Failed to run diceware : ".bright_red(),
                    "!Need to run on parent dir ( $HOME/[This Repo] ) ".bright_red()
                )
            });

        let dice = String::from_utf8_lossy(&diceware.stdout);

        let dice_split = dice.split("\n");

        let dice_vec: Vec<&str> = dice_split.collect();

        dice_vec
            .into_iter()
            .filter(|v| !v.is_empty())
            .map(|x| x.to_string())
            .collect()
    }
}
