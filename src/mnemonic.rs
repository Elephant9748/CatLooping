use crate::paper_backup::catch_stdin;
use bip39::{Language, Mnemonic, MnemonicType};
use colored::Colorize;

#[derive(Debug)]
pub struct Mnemonics<'mn> {
    count_words: Option<usize>,
    language: Option<&'mn str>,
}

impl<'mn> Mnemonics<'mn> {
    pub fn new(count_words: usize, language: &'mn str) -> Self {
        Self {
            count_words: Some(count_words),
            language: Some(language),
        }
    }
}

pub trait Bip39 {
    fn generate_mnemonic_word(&self) -> Option<String>;
}

impl<'mn> Bip39 for Mnemonics<'mn> {
    fn generate_mnemonic_word(&self) -> Option<String> {
        let mut phrase_result = String::new();
        match self.language? {
            "English" => {
                let mnemonic = Mnemonic::new(
                    MnemonicType::for_word_count(self.count_words?).unwrap(),
                    Language::English,
                );
                let phrase: &str = mnemonic.phrase();
                phrase_result.push_str(phrase);
            }
            "French" => {
                let mnemonic = Mnemonic::new(
                    MnemonicType::for_word_count(self.count_words?).unwrap(),
                    Language::French,
                );
                let phrase: &str = mnemonic.phrase();
                phrase_result.push_str(phrase);
            }
            "Italian" => {
                let mnemonic = Mnemonic::new(
                    MnemonicType::for_word_count(self.count_words?).unwrap(),
                    Language::Italian,
                );
                let phrase: &str = mnemonic.phrase();
                phrase_result.push_str(phrase);
            }
            "Japanese" => {
                let mnemonic = Mnemonic::new(
                    MnemonicType::for_word_count(self.count_words?).unwrap(),
                    Language::Japanese,
                );
                let phrase: &str = mnemonic.phrase();
                phrase_result.push_str(phrase);
            }
            "Korean" => {
                let mnemonic = Mnemonic::new(
                    MnemonicType::for_word_count(self.count_words?).unwrap(),
                    Language::Korean,
                );
                let phrase: &str = mnemonic.phrase();
                phrase_result.push_str(phrase);
            }
            "Spanish" => {
                let mnemonic = Mnemonic::new(
                    MnemonicType::for_word_count(self.count_words?).unwrap(),
                    Language::Spanish,
                );
                let phrase: &str = mnemonic.phrase();
                phrase_result.push_str(phrase);
            }
            "ChineseSimplified" => {
                let mnemonic = Mnemonic::new(
                    MnemonicType::for_word_count(self.count_words?).unwrap(),
                    Language::ChineseSimplified,
                );
                let phrase: &str = mnemonic.phrase();
                phrase_result.push_str(phrase);
            }
            "ChineseTraditional" => {
                let mnemonic = Mnemonic::new(
                    MnemonicType::for_word_count(self.count_words?).unwrap(),
                    Language::ChineseTraditional,
                );
                let phrase: &str = mnemonic.phrase();
                phrase_result.push_str(phrase);
            }
            _ => println!("Umatch language generate_mnemonic_word!"),
        }

        Some(phrase_result)
    }
}

pub fn mnemonic_menu_list() -> Vec<String> {
    let mut menu_result: Vec<String> = Vec::new();

    println!(
        "{}",
        "\n> Word count must be 12, 15, 18, 21, 24 !".bright_red()
    );
    println!(
        "{}",
        "-----------------------------------------".bright_yellow()
    );
    println!("{}", "1. Words12 ".bright_yellow());
    println!("{}", "2. Words15 ".bright_yellow());
    println!("{}", "3. Words18 ".bright_yellow());
    println!("{}", "4. Words21 ".bright_yellow());
    println!("{}", "5. Words24 ".bright_yellow());
    print!("{}", "\nchose : ".bright_yellow());

    let mut num = 0;
    let count = catch_stdin();

    match count.trim().parse() {
        Ok(val) => match val {
            1 => {
                num = 12;
            }
            2 => {
                num = 15;
            }
            3 => {
                num = 18;
            }
            4 => {
                num = 21;
            }
            5 => {
                num = 24;
            }
            _ => println!("{}", "unmatch val : ".bright_red()),
        },
        Err(err) => println!("{}{}", "unmatch count.trim().parse() !".bright_red(), err),
    }

    menu_result.push(num.to_string());

    println!("{}", "\n> Language ".bright_cyan());
    println!("{}", "-----------".bright_cyan());
    println!("{}", "1 English. ".bright_cyan());
    println!("{}", "2 ChineseSimplified. ".bright_cyan());
    println!("{}", "3 ChineseTraditional. ".bright_cyan());
    println!("{}", "4 French. ".bright_cyan());
    println!("{}", "5 Italian. ".bright_cyan());
    println!("{}", "6 Japanese. ".bright_cyan());
    println!("{}", "7 Korean. ".bright_cyan());
    println!("{}", "8 Spanish. ".bright_cyan());
    print!("{}", "\nChose : ".bright_cyan());

    let lang = catch_stdin();
    let mut lang_option = String::from("");

    match lang.trim().parse() {
        Ok(val) => match val {
            1 => {
                lang_option.push_str("English");
            }
            2 => {
                lang_option.push_str("ChineseSimplified");
            }
            3 => {
                lang_option.push_str("ChineseTraditional");
            }
            4 => {
                lang_option.push_str("French");
            }
            5 => {
                lang_option.push_str("Italian");
            }
            6 => {
                lang_option.push_str("Japanese");
            }
            7 => {
                lang_option.push_str("Korean");
            }
            8 => {
                lang_option.push_str("Spanish");
            }
            _ => println!("{}", "unmatch val lang".bright_red()),
        },
        Err(err) => println!("{}{}", "unmatch lang.trim().parse()".bright_red(), err),
    }

    menu_result.push(lang_option);

    menu_result
}
