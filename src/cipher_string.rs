use std::io::{Cursor, Read};

use base64_stream::{FromBase64Reader, ToBase64Reader};
use cipher_crypt::{Cipher, Rot13, Vigenere};
use colored::Colorize;
use openssl::sha::sha256;

use crate::{catch_stdin, get_secret_gpg};

pub fn main_convert() {
    println!("\n{}", "1. Txt-Base64-Rot13?".cyan());
    println!("{}", "2. Rot13-Base64-Txt?".cyan());
    println!("{}", "3. Txt-vigenere?".cyan());
    println!("{}", "4. vigenere_Txt?".cyan());
    print!("\n{}", "Chose Option: ".bright_green());
    let option_string = catch_stdin();

    match option_string {
        val if val == "1" => {
            print!("\n{}", "> Input string: ".cyan());
            let input1 = catch_stdin();
            print_txt_base64_rot13(input1.as_str())
        }
        val if val == "2" => {
            print!("\n{}", "> Input string: ".cyan());
            let input2 = catch_stdin();
            print_rot13_base64_txt(input2.as_str())
        }
        val if val == "3" => {
            print!("\n{}", "> Input String: ".cyan());
            let input3 = catch_stdin();
            print!("\n{}", "> key String: ".cyan());
            let input4 = catch_stdin();
            print_txt_vigenere(input3.as_str(), input4.as_str());
        }
        val if val == "4" => {
            print!("\n{}", "> Input String: ".cyan());
            let a = catch_stdin();
            print!("\n{}", "> key String: ".cyan());
            let b = catch_stdin();
            print_vigenere_txt(a.as_str(), b.as_str());
        }
        _ => println!("{}", "> Option not available!".bright_red()),
    }
}

fn print_rot13_base64_txt(val: &str) {
    let raw_txt_copy = val;
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!("{}{}", "--> Rot13  : ".bright_green(), raw_txt_copy);
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!(
        "{}{}",
        "--> Base64 : ".bright_green(),
        from_rot13(val).bright_yellow()
    );
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!(
        "{}{}",
        "--> Txt     : ".bright_green(),
        from_rot13_base64_txt(val).unwrap().bright_green()
    );
}

fn print_txt_base64_rot13(val: &str) {
    let raw_txt_copy = val;
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!("{}{}", "--> Text   : ".bright_green(), raw_txt_copy);
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!(
        "{}{}",
        "--> Base64 : ".bright_green(),
        to_base64(val).unwrap().bright_yellow()
    );
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!(
        "{}{}",
        "--> Rot13   : ".bright_green(),
        to_txt_base64_rot13(val).unwrap().bright_green()
    );
}

fn print_txt_vigenere(val: &str, key: &str) {
    let raw_txt_copy = val;
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!("{}{}", "--> Text     : ".bright_green(), raw_txt_copy);
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!(
        "{}{}",
        "--> vigenere : ".bright_green(),
        to_vigenere(val, key).bright_yellow()
    );
}

fn print_vigenere_txt(val: &str, key: &str) {
    let raw_txt_copy = val;
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!("{}{}", "--> Vigenere   : ".bright_green(), raw_txt_copy);
    println!("{}", "|".bright_green());
    println!("{}", "|".bright_green());
    println!(
        "{}{}",
        "--> Text       : ".bright_green(),
        from_vigenere(val, key).bright_yellow()
    );
}

pub fn to_txt_base64_rot13(val: &str) -> Result<String, String> {
    let mut reader = ToBase64Reader::new(Cursor::new(val.as_bytes().to_vec()));
    let mut from_base64 = String::new();
    reader.read_to_string(&mut from_base64).unwrap();

    let from_rot13 = from_rot13(from_base64.as_str());
    match from_rot13 {
        val if !val.is_empty() => Ok(val),
        _ => Err("Error cant generate strin from rot13!.".to_string()),
    }
}

pub fn from_rot13_base64_txt(val: &str) -> Result<String, String> {
    let from_rot13 = from_rot13(val);
    let mut reader = FromBase64Reader::new(Cursor::new(from_rot13.as_bytes().to_vec()));
    let mut from_base64 = String::new();
    reader.read_to_string(&mut from_base64).unwrap();

    match from_base64 {
        val if !val.is_empty() => Ok(val),
        _ => Err("Error cant generate strin from rot13!.".to_string()),
    }
}

pub fn to_base64(val: &str) -> Result<String, String> {
    let mut reader = ToBase64Reader::new(Cursor::new(val));
    let mut to_base64 = String::new();
    reader.read_to_string(&mut to_base64).unwrap();

    let to_base64_copy = to_base64.clone();

    match to_base64_copy {
        y if !y.is_empty() => Ok(to_base64),
        _ => Err("Error cant generate base64 from string!".to_string()),
    }
}

#[allow(dead_code)]
pub fn from_base64(val: &str) -> Result<String, String> {
    let mut reader = FromBase64Reader::new(Cursor::new(val));
    let mut from_base64 = String::new();
    reader.read_to_string(&mut from_base64).unwrap();

    let from_base64_copy = from_base64.clone();

    if !from_base64_copy.is_empty() {
        Ok(from_base64)
    } else {
        Err("Error cant generate string from base64!".to_string())
    }
}

pub fn to_sha256(file: &str) -> Vec<String> {
    let get_gpg_from_file = get_secret_gpg(file);

    let hasher = sha256(get_gpg_from_file.as_bytes());
    let hash_value = hex::encode(hasher);

    let get_gpg_from_file_split = get_gpg_from_file.split("\n");
    let hash_vec: Vec<&str> = get_gpg_from_file_split.collect();

    println!("{}{}", "> Hash sha256 : ".bright_yellow(), hash_value);
    for line in hash_vec {
        if line == "" {
            println!("");
        }

        if line == "-----END PGP MESSAGE-----" {
            println!("{}", line.green());
            break;
        }

        println!("{}", line.green());
    }

    let hash_value_char: Vec<char> = hash_value.chars().collect();
    let mut short_hash = String::new();

    let mut y = 0;
    while y < hash_value_char.len() {
        short_hash.push(hash_value_char[y]);
        if y == 22 {
            break;
        }
        y += 1;
    }

    vec![short_hash, hash_value, get_gpg_from_file]
}

pub fn to_vigenere(val: &str, key: &str) -> String {
    let key_vigenere = Vigenere::new(String::from(key));
    key_vigenere.encrypt(val).unwrap()
}

pub fn from_vigenere(val: &str, key: &str) -> String {
    let key_vigenere = Vigenere::new(String::from(key));
    key_vigenere.decrypt(val).unwrap()
}

#[allow(dead_code)]
pub fn to_rot13(val: &str) -> String {
    Rot13::encrypt(val)
}

pub fn from_rot13(val: &str) -> String {
    Rot13::decrypt(val)
}
