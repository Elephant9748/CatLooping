pub mod lib {

    use base64_stream::{FromBase64Reader, ToBase64Reader};
    use bip39::{Language, Mnemonic, MnemonicType};
    use chrono::prelude::*;
    use cipher_crypt::{Cipher, Rot13, Vigenere};
    use colored::Colorize;
    use openssl::sha::sha256;
    use progress_bar::*;
    use qr2term::*;
    use qrcode_png::{Color as ColorQr, QrCode, QrCodeEcc};
    use std::{
        fs::File,
        fs::OpenOptions,
        io::{self, BufRead, Cursor, Read, Write},
        path::Path,
        process::{Command, Stdio},
        thread::sleep,
        time::Duration,
    };

    pub enum Menu {
        Help,
        Eff(usize),
        EffLock(usize),
        Diceware(String),
        DicewareLock(String),
        Notenum(String),
        MnemonicGen(usize, String),
        MnemonicGenLock(usize, String),
        LockString(String),
        ToFile(String),
        QrOnly(String),
        Unlock,
        Convert,
    }

    macro_rules! clear_screen {
        () => {
            std::process::Command::new("clear").status().unwrap();
        };
    }

    macro_rules! exit_this {
        () => {
            std::process::exit(0);
        };
    }

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
            let diceware = Command::new("bin/diceware")
                .args(&[
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
                .expect(
                    format!(
                        "{}{}",
                        "> Failed to run diceware : ".bright_red(),
                        "!Need to run on parent dir ( $HOME/[This Repo] ) ".bright_red()
                    )
                    .as_str(),
                );

            let dice = String::from_utf8_lossy(&diceware.stdout);

            let dice_split = dice.split("\n");

            let dice_vec: Vec<&str> = dice_split.collect();

            dice_vec
                .into_iter()
                .filter(|v| v.to_string() != "")
                .map(|x| x.to_string())
                .collect()
        }
    }

    #[derive(Debug)]
    pub struct Eff {
        n_value: Option<usize>,
    }

    impl Eff {
        pub fn new(n_value: usize) -> Self {
            let value = n_value;
            Self {
                n_value: Some(value),
            }
        }
    }

    pub trait Effdefault {
        fn generate_eff(&self) -> Option<String>;
    }

    impl Effdefault for Eff {
        fn generate_eff(&self) -> Option<String> {
            let mut words: Vec<String> = Vec::new();

            let mut words_string = String::new();

            while words.len() < self.n_value.unwrap() {
                let word = eff_wordlist::large::random_word();
                words.push(word.to_string());
                words_string.push_str(&word.to_string());
                words_string.push('-');
            }

            words_string.pop();

            Some(words_string)
        }
    }

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
                lg if lg == "English" => {
                    let mnemonic = Mnemonic::new(
                        MnemonicType::for_word_count(self.count_words?).unwrap(),
                        Language::English,
                    );
                    let phrase: &str = mnemonic.phrase();
                    phrase_result.push_str(phrase);
                }
                lg if lg == "French" => {
                    let mnemonic = Mnemonic::new(
                        MnemonicType::for_word_count(self.count_words?).unwrap(),
                        Language::French,
                    );
                    let phrase: &str = mnemonic.phrase();
                    phrase_result.push_str(phrase);
                }
                lg if lg == "Italian" => {
                    let mnemonic = Mnemonic::new(
                        MnemonicType::for_word_count(self.count_words?).unwrap(),
                        Language::Italian,
                    );
                    let phrase: &str = mnemonic.phrase();
                    phrase_result.push_str(phrase);
                }
                lg if lg == "Japanese" => {
                    let mnemonic = Mnemonic::new(
                        MnemonicType::for_word_count(self.count_words?).unwrap(),
                        Language::Japanese,
                    );
                    let phrase: &str = mnemonic.phrase();
                    phrase_result.push_str(phrase);
                }
                lg if lg == "Korean" => {
                    let mnemonic = Mnemonic::new(
                        MnemonicType::for_word_count(self.count_words?).unwrap(),
                        Language::Korean,
                    );
                    let phrase: &str = mnemonic.phrase();
                    phrase_result.push_str(phrase);
                }
                lg if lg == "Spanish" => {
                    let mnemonic = Mnemonic::new(
                        MnemonicType::for_word_count(self.count_words?).unwrap(),
                        Language::Spanish,
                    );
                    let phrase: &str = mnemonic.phrase();
                    phrase_result.push_str(phrase);
                }
                lg if lg == "ChineseSimplified" => {
                    let mnemonic = Mnemonic::new(
                        MnemonicType::for_word_count(self.count_words?).unwrap(),
                        Language::ChineseSimplified,
                    );
                    let phrase: &str = mnemonic.phrase();
                    phrase_result.push_str(phrase);
                }
                lg if lg == "ChineseTraditional" => {
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

    pub fn menu_option(menu_list: Menu) {
        match menu_list {
            Menu::Help => get_help(),
            Menu::Diceware(arg) => {
                let dice = Dice::new(arg.parse::<u32>().unwrap(), "minilock", "-");
                let diceware = dice.generate_wordlist();

                println!("{:?}", dice);

                println!("{}", "> diceware".bright_cyan());
                println!("{}", "---------".bright_cyan());
                println!(
                    "{}{}",
                    "entropy   : ".cyan(),
                    diceware.find_entropy().unwrap()
                );
                println!(
                    "{}{}\n",
                    "passphrase: ".green(),
                    diceware
                        .find_passphrase()
                        .unwrap()
                        .color("white")
                        .on_color("black")
                        .italic()
                );
            }
            Menu::DicewareLock(arg) => {
                let phrase = Dice::new(arg.parse::<u32>().unwrap(), "minilock", "-");
                let passphrase = phrase.generate_wordlist();

                println!("{}", "> diceware".bright_cyan());
                println!("{}", "---------".bright_cyan());
                println!(
                    "{}{}",
                    "entropy   : ".cyan(),
                    passphrase.find_entropy().unwrap()
                );
                println!(
                    "{}{}\n",
                    "passphrase: ".green(),
                    passphrase
                        .find_passphrase()
                        .unwrap()
                        .color("white")
                        .on_color("black")
                        .italic()
                );

                print!("{}", "> do you want to continue [y/n]: ".bright_yellow());
                let forward_this = catch_stdin();
                if forward_this == "y" || forward_this == "Y" {
                    println!("{:?}", passphrase.find_passphrase().unwrap());
                    store_tofile(passphrase.find_passphrase().unwrap().to_string());

                    println!("{}", gpg_encrypt().unwrap().bright_green());

                    let hash = to_sha256("secret.gpg");

                    println!("{}{}", "> Hash thing: ".bright_red(), hash[0]);
                    qrcode_generate_to_file(hash[2].as_str(), hash[1].as_str(), hash[0].as_str());

                    println!(
                        "{}",
                        shred_helper_files(["secret.gpg", "frost"].to_vec())
                            .unwrap()
                            .bright_green()
                    );
                } else {
                    exit_this!();
                }
            }
            Menu::Unlock => unlock_qrcode(),
            Menu::Eff(arg) => {
                let init_eff = Eff::new(arg);
                println!("\neff wordlist");
                println!("------------");
                println!(
                    "{}{}\n",
                    "Output: ".green(),
                    init_eff.generate_eff().unwrap()
                );
            }
            Menu::EffLock(arg) => {
                let init_eff = Eff::new(arg);
                let eff = init_eff.generate_eff().unwrap();

                println!("\neff wordlist");
                println!("------------");
                println!("{}{}\n", "Output: ".green(), eff);

                print!("{}", "> do you want to continue [y/n]: ".bright_yellow());
                let forward_this = catch_stdin();
                match forward_this {
                    x if x == "y" || x == "Y" => {
                        store_tofile(eff);

                        println!("{}", gpg_encrypt().unwrap().bright_green());

                        let hash = to_sha256("secret.gpg");

                        println!("{}{}", "> Hash thing: ".bright_red(), hash[0]);
                        qrcode_generate_to_file(
                            hash[2].as_str(),
                            hash[1].as_str(),
                            hash[0].as_str(),
                        );

                        println!(
                            "{}",
                            shred_helper_files(["secret.gpg", "frost"].to_vec())
                                .unwrap()
                                .bright_green()
                        );
                    }
                    _ => {
                        exit_this!();
                    }
                }
            }
            Menu::Notenum(arg) => {
                println!(
                    "{} {}",
                    arg.bright_red(),
                    "> Menu Argument not available please check help: --help".bright_yellow()
                );
            }
            Menu::Convert => main_convert(),
            Menu::MnemonicGen(arg1, arg2) => {
                let init_mnemonic = Mnemonics::new(arg1, arg2.as_str());
                let out = init_mnemonic.generate_mnemonic_word();
                println!(
                    "\n{}{}",
                    "> Phrase: ".bright_green(),
                    out.unwrap().bright_cyan()
                );
            }
            Menu::MnemonicGenLock(arg1, arg2) => {
                let init_mnemonic = Mnemonics::new(arg1, arg2.as_str());
                let mnemoniclock_val = init_mnemonic.generate_mnemonic_word().unwrap();
                println!(
                    "\n{}{}",
                    "> Phrase: ".bright_green(),
                    mnemoniclock_val.bright_cyan()
                );

                print!("{}", "> do you want to continue [y/n]: ".bright_yellow());
                let forward_this = catch_stdin();
                match forward_this {
                    x if x == "y" || x == "Y" => {
                        store_tofile(mnemoniclock_val);

                        println!("{}", gpg_encrypt().unwrap().bright_green());

                        let hash = to_sha256("secret.gpg");

                        println!("{}{}", "> Hash thing: ".bright_red(), hash[0]);
                        qrcode_generate_to_file(
                            hash[2].as_str(),
                            hash[1].as_str(),
                            hash[0].as_str(),
                        );

                        println!(
                            "{}",
                            shred_helper_files(["secret.gpg", "frost"].to_vec())
                                .unwrap()
                                .bright_green()
                        );
                    }
                    _ => {
                        exit_this!();
                    }
                }
            }
            Menu::LockString(arg) => {
                print!("{}", "> do you want to continue [y/n]: ".bright_yellow());
                let forward_this = catch_stdin();
                match forward_this {
                    x if x == "y" || x == "Y" => {
                        store_tofile(arg);

                        println!("{}", gpg_encrypt().unwrap().bright_green());

                        let hash = to_sha256("secret.gpg");

                        println!("{}{}", "> Hash thing: ".bright_red(), hash[0]);
                        qrcode_generate_to_file(
                            hash[2].as_str(),
                            hash[1].as_str(),
                            hash[0].as_str(),
                        );

                        println!(
                            "{}",
                            shred_helper_files(["secret.gpg", "frost"].to_vec())
                                .unwrap()
                                .bright_green()
                        );
                    }
                    _ => {
                        exit_this!();
                    }
                }
            }
            Menu::ToFile(arg) => {
                print!("{}", "> do you want to continue [y/n]: ".bright_yellow());
                let forward_this = catch_stdin();
                match forward_this {
                    x if x == "y" || x == "Y" => {
                        qrcode_generate_to_file2(arg.as_str(), "qrfile");
                    }
                    _ => {
                        exit_this!();
                    }
                }
            }
            Menu::QrOnly(arg) => {
                print!("{}", "> do you want to continue [y/n]: ".bright_yellow());
                let forward_this = catch_stdin();
                match forward_this {
                    x if x == "y" || x == "Y" => {
                        qrcode_generate_to_file(arg.as_str(), "qr0", "qr");
                    }
                    _ => {
                        exit_this!();
                    }
                }
            }
        }
    }

    pub fn catch_stdin() -> String {
        let mut input = String::new();

        let _ = io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Something wrong with stdin().read_line() !");

        input.trim().to_string()
    }

    pub fn gpg_encrypt() -> Result<String, String> {
        let gpg = Command::new("gpg")
            .args(&[
                "-o",
                "secret.gpg",
                "--symmetric",
                "--s2k-mode",
                "3",
                "--s2k-count",
                "65011712",
                "--s2k-digest-algo",
                "SHA512",
                "--cipher-algo",
                "AES256",
                "--armor",
                "frost",
            ])
            .stdout(Stdio::piped())
            .output()
            .expect("> gpg_encrypt() failed!");

        let gpg_utf8 = String::from_utf8_lossy(&gpg.stdout);
        let gpg_utf8_err = String::from_utf8_lossy(&gpg.stderr);

        if gpg_utf8.is_empty() {
            Ok(format!("> gpg_encrypt successfully."))
        } else {
            Err(format!(
                "> something wrong with gpg_utf8_err! : {}",
                gpg_utf8_err
            ))
        }
    }

    pub fn store_tofile(store_val: String) {
        let store_val_copy = store_val.clone();

        validate_passphrase(store_val_copy);

        let path = Path::new("frost");
        let show_path = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("> couldn't create path {}: {}", show_path, why),
            Ok(file) => file,
        };

        match file.write_all(format!("{}\n", store_val).as_bytes()) {
            Err(why) => panic!("> couldn't write to {}: {}", show_path, why),
            Ok(_) => println!("{}{}", "> successfully wrote to ".purple(), show_path),
        };
    }

    pub fn validate_passphrase(val: String) -> String {
        clear_screen!();
        loop {
            // some kind a time sleep
            println!(
                "{}",
                "> type '--show' to look passphrase previously".bright_yellow()
            );
            print!("{}", "> please validate passphrase : ".bright_blue());
            let check = catch_stdin();
            clear_screen!();

            if check == "" {
                let val_copy = val.clone();
                validate_passphrase(val_copy);
                break;
            }

            if check == val {
                println!("{}", "> validate successfully.".bright_green());
                sleep(Duration::from_millis(500));
                break;
            }

            if check == "--show" {
                clear_screen!();
                println!(
                    "{}{}",
                    "> what store passphrase: ".bright_green(),
                    val.yellow()
                );
                // some kind a time sleep
            }
        }
        val.to_string()
    }

    pub fn shred_helper_files(val: Vec<&str>) -> Result<String, String> {
        let mut shred_args: Vec<&str> = Vec::new();
        shred_args.push("-vuzn");
        shred_args.push("20");

        let mut what_file_shreding = String::new();
        for v in val {
            what_file_shreding.push_str(v);
            what_file_shreding.push_str(", ");
            shred_args.push(v);
        }

        println!(
            "{}{}{}",
            "> Shreding ".magenta(),
            what_file_shreding.magenta(),
            " ,20: ".bright_yellow()
        );

        let shred = Command::new("shred")
            .args(&shred_args)
            .stdout(Stdio::piped())
            .output()
            .expect("> shred_helper_files failed!");

        let shred_utf8 = String::from_utf8_lossy(&shred.stdout);
        let shred_utf8_err = String::from_utf8_lossy(&shred.stderr);

        let shred_copy_split = shred_utf8_err.split("\n");
        let shred_copy_collect: Vec<&str> = shred_copy_split.collect();
        process_bar(shred_copy_collect.len());

        if shred_utf8.is_empty() {
            Ok(format!("{}", "> shred successfully. ".green()))
        } else {
            Err(format!("> something wrong with shreding files"))
        }
    }

    pub fn reset_gpg_agent() -> Result<String, String> {
        let mut option: Vec<&str> = Vec::new();
        option.push("--kill");
        option.push("all");

        let run = Command::new("gpg")
            .args(&option)
            .stdout(Stdio::piped())
            .output()
            .expect("failed reset_gpg_agent!");

        let run_utf8 = String::from_utf8_lossy(&run.stdout);
        let run_utf8_err = String::from_utf8_lossy(&run.stderr);

        if run_utf8.is_empty() {
            Ok(format!("{}", "> reset gpg agent ok. ".green()))
        } else {
            Err(format!(
                "> something wrong with reset_gpg_agent: {}",
                run_utf8_err
            ))
        }
    }

    pub fn process_bar(val: usize) {
        init_progress_bar(val);
        set_progress_bar_action("*shreding", Color::Magenta, Style::Normal);

        let mut i = 0;
        while i < val {
            sleep(Duration::from_millis(15));
            inc_progress_bar();
            i += 1;
        }
        finalize_progress_bar();
    }

    // Need better generate qrcode
    pub fn qrcode_generate_to_file(val: &str, val2: &str, val3: &str) {
        let utc: DateTime<Utc> = Utc::now();
        let utc_to_png = utc.format("%m%d%y_%H%M").to_string();
        print!("{}", "> Name your qrcode file: ".bright_yellow());

        let name_png = catch_stdin();
        let name_png_print = format!("qrcode/{}_{}_{}.png", val2, utc_to_png, name_png);
        let name_png_print_copy = name_png_print.clone();
        let mut qrcode = QrCode::new(val.as_bytes(), QrCodeEcc::Medium).unwrap();

        qrcode.margin(50);
        qrcode.zoom(10);

        let buffer = qrcode.generate(ColorQr::Grayscale(0, 255)).unwrap();
        std::fs::write(name_png_print, buffer).expect(
            format!(
                "{}",
                ">Something wrong with qrcode_generate write file".red()
            )
            .as_str(),
        );

        print_qr(val).unwrap();
        println!(
            "{}{}",
            "> qrcode location : ".green(),
            name_png_print_copy.magenta()
        );

        let status_short =
            qrcode_with_short_hash(val2, utc_to_png.as_str(), name_png.as_str(), val3).unwrap();

        println!("{}", status_short);
    }

    // for pure file to qrcode
    pub fn qrcode_generate_to_file2(file: &str, val2: &str) {
        let utc: DateTime<Utc> = Utc::now();
        let utc_to_png = utc.format("%m%d%y_%H%M").to_string();
        print!("{}", "> Name your qrcode file: ".bright_yellow());

        let content_file = std::fs::read_to_string(file).expect("read_file failed !");

        let name_png = catch_stdin();
        let name_png_print = format!("qrcode/{}_{}_{}.png", val2, utc_to_png, name_png);
        let name_png_print_copy = name_png_print.clone();
        let mut qrcode = QrCode::new(content_file.as_bytes(), QrCodeEcc::Medium).unwrap();

        qrcode.margin(50);
        qrcode.zoom(10);

        let buffer = qrcode.generate(ColorQr::Grayscale(0, 255)).unwrap();
        std::fs::write(name_png_print, buffer).expect(
            format!(
                "{}",
                ">Something wrong with qrcode_generate write file".red()
            )
            .as_str(),
        );

        print_qr(&content_file).unwrap();
        println!(
            "{}{}",
            "> qrcode location : ".green(),
            name_png_print_copy.magenta()
        );
        let qrcode_shrink = qrcode_with_short2(utc_to_png.as_str(), name_png.as_str()).unwrap();

        println!("{}", qrcode_shrink);
    }

    fn qrcode_with_short_hash(
        hash: &str,
        utc_time: &str,
        name_png: &str,
        short_hash: &str,
    ) -> Result<String, String> {
        let qrcode_short = Command::new("convert")
            .args(&[
                format!("qrcode/{}_{}_{}.png", hash, utc_time, name_png).as_str(),
                "-gravity",
                "center",
                "-scale",
                "200%",
                "-extent",
                "100%",
                "-scale",
                "100%",
                "-gravity",
                "south",
                "-font",
                "/usr/share/fonts/truetype/noto/NotoMono-Regular.ttf",
                "-pointsize",
                "24",
                "-fill",
                "black",
                "-draw",
                format!("text 0,50 '{}-{}'", short_hash, name_png).as_str(),
                format!("qrcode/{}_{}_{}.png", short_hash, utc_time, name_png).as_str(),
            ])
            .stdout(Stdio::piped())
            .output()
            .expect("> somthing wrong with hashlib_python!");

        let qrcode_short_utf8 = String::from_utf8_lossy(&qrcode_short.stdout);
        if qrcode_short_utf8.is_empty() {
            Ok(format!(
                "{}",
                "> qrcode_with_short_hash successfully.".green()
            ))
        } else {
            Err(format!(
                "{}",
                "> somthing wrong with qrcode_with_short_hash!".bright_red()
            ))
        }
    }

    fn qrcode_with_short2(utc_time: &str, name_png: &str) -> Result<String, String> {
        let qrcode_short = Command::new("convert")
            .args(&[
                format!("qrcode/qrfile_{}_{}.png", &utc_time, &name_png).as_str(),
                "-gravity",
                "center",
                "-scale",
                "200%",
                "-extent",
                "100%",
                "-scale",
                "100%",
                "-gravity",
                "south",
                "-font",
                "/usr/share/fonts/truetype/noto/NotoMono-Regular.ttf",
                "-pointsize",
                "24",
                "-fill",
                "black",
                "-draw",
                format!("text 0,50 '{}-{}'", utc_time, name_png).as_str(),
                format!("qrcode/qrfile_tag_{}_{}.png", utc_time, name_png).as_str(),
            ])
            .stdout(Stdio::piped())
            .output()
            .expect("> somthing wrong with convert image!!");

        let qrcode_short_utf8 = String::from_utf8_lossy(&qrcode_short.stdout);
        if qrcode_short_utf8.is_empty() {
            Ok(format!(
                "{}",
                "> qrcode_with_name_tag successfully.".green()
            ))
        } else {
            Err(format!(
                "{}",
                "> somthing wrong with qrcode_with_name_tag!".bright_red()
            ))
        }
    }

    pub fn get_secret_gpg(string_path: &str) -> String {
        let mut bucket_val = String::new();
        if let Ok(lines) = read_a_file(string_path) {
            for line in lines {
                if let Ok(val) = line {
                    bucket_val.push_str(val.as_str());
                    bucket_val.push_str("\n");
                }
            }
        }
        bucket_val
    }

    fn read_a_file<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        T: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn unlock_qrcode() {
        println!(
            "\n{}",
            "If no input go to default directory qrcode/".bright_yellow()
        );
        println!("{}", "By press Enter!".bright_yellow());
        print!("{}", "Input path of qrcode : ".bright_green());
        let path_stdin_val = catch_stdin();

        let mut path_stdin_val_mut = String::new();
        if path_stdin_val.is_empty() {
            path_stdin_val_mut.push_str("qrcode");
        } else {
            path_stdin_val_mut.push_str(path_stdin_val.as_str());
        }

        clear_screen!();
        let list_of_qrcode = Command::new("ls")
            .args(&["-a", format!("{}", path_stdin_val_mut).as_str()])
            .stdout(Stdio::piped())
            .output()
            .expect(
                format!(
                    "{}",
                    "> somthing wrong with list_of_qrcode. (qrcode directory not found)"
                        .bright_red()
                )
                .as_str(),
            );

        let list_of_qrcode_utf8 = String::from_utf8_lossy(&list_of_qrcode.stdout);

        if list_of_qrcode_utf8.is_empty() {
            panic!("{}", "No qrcode directory".bright_red());
        }

        let list_of_qrcode_split = list_of_qrcode_utf8.split("\n");

        let list_of_qrcode_vec: Vec<&str> = list_of_qrcode_split.collect();

        let output: Vec<&str> = list_of_qrcode_vec
            .clone()
            .into_iter()
            .filter(|&x| x != "..".to_string() && x != ".".to_string() && x != "".to_string())
            .collect::<Vec<&str>>();

        println!();
        println!("{}", "List of orcode".yellow());
        println!("{}", "--------------".yellow());

        let mut index = Some(0);

        while let Some(i) = index {
            if i == output.len() {
                index = None;
            } else if i % 2 == 0 {
                println!("{}. {}", i, output[i].bright_purple());
                index = Some(i + 1);
            } else {
                println!("{}. {}", i, output[i].bright_yellow());
                index = Some(i + 1);
            }
        }

        print!(
            "{}",
            "\n> chose file name by index or name: ".bright_green()
        );

        let chose = catch_stdin();
        let mut chose_copy = chose.clone();

        let out_chose = stdin_check_numeric(chose.as_str());

        let mut chose_split: Vec<&str> = Vec::new();

        let mut path_name = String::new();
        if out_chose {
            let index_path_name = chose.trim().parse::<usize>().unwrap();
            path_name.push_str(output[index_path_name]);
        } else {
            if chose.contains("[") && chose.contains("]") && chose.contains(",") {
                chose_copy.remove(chose_copy.len() - 1);
                chose_copy.remove(0);
                chose_split = chose_copy.split(",").collect();
            } else {
                path_name.push_str(format!("{}", chose).as_str());
            }
        }

        if chose_split.is_empty() {
            scan_qrcode(path_name.as_str(), path_stdin_val_mut.as_str());

            print!("{}", "> Do yo want to show passphrase[y/n]: ".bright_red());
            let confirm = catch_stdin();
            if confirm == "Y" || confirm == "y" {
                println!(
                    "{}{}",
                    "> passphrase: ".bright_green(),
                    gpg_decrypt().unwrap().bright_yellow()
                );
            } else {
                println!("{}{}", "> passphrase: ".bright_green(), "Nope".bright_red());
            }

            shred_helper_files(["qrcode_encode.gpg"].to_vec())
                .unwrap()
                .bright_green();
        } else {
            let output_copy = output.clone();
            let index_chose_split: Vec<usize> = chose_split
                .iter()
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect();

            for el in index_chose_split.iter() {
                println!(
                    "{}{}",
                    "> ".bright_purple(),
                    output_copy[*el].bright_purple()
                );
            }
            sleep(Duration::from_millis(500));

            print!("{}", "> Do yo want to show passphrase[y/n]: ".bright_red());
            let confirm = catch_stdin();
            if confirm == "Y" || confirm == "y" {
                for path_n in index_chose_split.iter() {
                    scan_qrcode(output_copy[*path_n], path_stdin_val_mut.as_str());

                    println!(
                        "{}{}",
                        "> ".bright_purple(),
                        output_copy[*path_n].bright_purple()
                    );
                    println!(
                        "{}{}",
                        "> passphrase: ".bright_green(),
                        gpg_decrypt().unwrap().bright_yellow()
                    );

                    shred_helper_files(["qrcode_encode.gpg"].to_vec())
                        .unwrap()
                        .bright_green();
                }
            } else {
                println!("{}{}", "> passphrase: ".bright_green(), "Nope".bright_red());
            }
        }
        println!("{}", reset_gpg_agent().unwrap().bright_green());
    }

    pub fn stdin_check_numeric(val: &str) -> bool {
        let chars: Vec<char> = val.chars().collect();
        let chars_copy = chars.clone();
        let mut numeric = 0;
        for char in chars {
            if char.is_digit(10) {
                numeric += 1;
            }
        }

        if numeric == chars_copy.len() {
            true
        } else {
            false
        }
    }

    fn scan_qrcode(name_of_file: &str, path_of_file: &str) {
        let mut path_of_file_mut = String::new();
        if path_of_file.is_empty() {
            path_of_file_mut.push_str("qrcode");
        } else {
            path_of_file_mut.push_str(path_of_file);
        }

        let qrcode_name_location = format!("{}/{}", path_of_file_mut, name_of_file);
        let zbar = Command::new("zbarimg")
            .args(&[
                "--nodisplay",
                "--nodbus",
                "--quiet",
                qrcode_name_location.as_str(),
            ])
            .stdout(Stdio::piped())
            .output()
            .expect(format!("{}", "somthing wrong with zbar piped()".bright_red()).as_str());

        let zbar_utf8 = String::from_utf8_lossy(&zbar.stdout);
        let zbar_utf8_replace = zbar_utf8.replace("QR-Code:", "");
        let zbar_utf8_split = zbar_utf8_replace.split("\n");
        let zbar_utf8_vec: Vec<&str> = zbar_utf8_split.collect();

        // write to qrcode_decode.gpg
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("qrcode_encode.gpg")
            .unwrap();

        for line in zbar_utf8_vec.into_iter() {
            writeln!(&mut file, "{}", line)
                .expect(format!("{}", "> something wrong with writeln!".bright_red()).as_str());
        }

        let gpgterm = get_secret_gpg("qrcode_encode.gpg");
        println!("> {}", gpgterm.bright_yellow());
    }

    pub fn gpg_decrypt() -> Result<String, String> {
        let gpg = Command::new("gpg")
            .args(&["--decrypt", "qrcode_encode.gpg"])
            .stdout(Stdio::piped())
            .output()
            .expect("> gpg_decrypt() failed!");

        let gpg_utf8 = String::from_utf8_lossy(&gpg.stdout);
        let gpg_utf8_err = String::from_utf8_lossy(&gpg.stderr);
        let gpg_utf8_split = gpg_utf8.split("\n");
        let gpg_utf8_vec: Vec<&str> = gpg_utf8_split.collect();

        if !gpg_utf8.is_empty() {
            Ok(format!("{}", gpg_utf8_vec[0]))
        } else {
            Err(format!(
                "{}{}",
                "> something wrong with gpg_utf8_err gpg_decrypt() : ".bright_red(),
                gpg_utf8_err
            ))
        }
    }

    fn main_convert() {
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

    pub fn to_rot13(val: &str) -> String {
        Rot13::encrypt(val)
    }

    pub fn from_rot13(val: &str) -> String {
        Rot13::decrypt(val)
    }

    pub fn get_help() {
        println!("\nrequire: ");
        println!("       - rust-diceware binary from crate.io manually installed");
        println!("");
        println!("usage: paper_backup [--help] [--eff]");
        println!("");
        println!("option: ");
        println!("       --help           :  Help command!");
        println!("       --eff            :  Generate Eff random wordlist");
        println!("       --eff-lock       :  Generate paper backup with Eff random wordlist");
        println!("       --diceware       :  Generate passphrase using diceware crate");
        println!("       --diceware-lock  :  Generate qrcoode paper backup with --diceware");
        println!("       --mnemonic       :  Generate passphrase using tiny-bip39 crate");
        println!("       --mnemonic-lock  :  Generate qrcode paper backup using tiny-bip39 crate");
        println!("       --unlock         :  Unlock qrcode from directory qrcode/");
        println!("       --lock-string    :  Generate qrcode paper backup from string input");
        println!("       --qrcode-no-pgp  :  Generate qrcode only no pgp");
        println!("       --to-file        :  Generate qrcode only no pgp from file");
        println!("       --convert        :  Convertion string to ?\n");
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
}

#[cfg(test)]
mod tests {
    use crate::lib::*;

    #[test]
    fn test_to_viginere() {
        let key = "bishon";
        let value = "MESSAGE";
        assert_eq!("NMKZOTF", to_vigenere(value, key));
    }

    #[test]
    fn test_from_viginere() {
        let key = "bishon";
        let value = "NMKZOTF";
        assert_eq!("MESSAGE", from_vigenere(value, key));
    }

    #[test]
    fn test_to_txt_base64_rot13() {
        let txt = "HELLO!";
        let real = to_txt_base64_rot13(txt).unwrap();
        assert_eq!("FRIZGR8u", real);
    }

    #[test]
    fn test_from_rot13_base64_txt() {
        let txt = "FRIZGR8u";
        let real = from_rot13_base64_txt(txt).unwrap();
        assert_eq!("HELLO!", real);
    }

    #[test]
    fn test_to_base64() {
        let base64_txt = "HELLO!";
        let real = to_base64(base64_txt).unwrap();
        assert_eq!("SEVMTE8h", real);
    }

    #[test]
    fn test_from_base64() {
        let base64_txt = "SEVMTE8h";
        let real = from_base64(base64_txt).unwrap();
        assert_eq!("HELLO!", real);
    }

    #[test]
    fn test_from_rot13() {
        let txt = "grkg_sebz_ebg13";
        assert_eq!(from_rot13(&txt), "text_from_rot13");
    }
    #[test]
    fn test_to_rot13() {
        let txt = "text_to_rot13";
        assert_eq!(to_rot13(txt), "grkg_gb_ebg13");
    }

    #[test]
    fn test_diceware_generate() {
        let dice_init = Dice::new(1, "minilock", "-");
        let dice: Vec<String> = dice_init.generate_wordlist();
        let mut val = false;
        for el in dice {
            let n = stdin_check_numeric(el.as_str());
            if !n {
                val = !n
            } else {
                val = n
            }
        }
        assert!(val);
    }

    #[test]
    fn test_generate_eff_word() {
        let init_eff = Eff::new(1);
        let eff = init_eff.generate_eff();
        let n = stdin_check_numeric(eff.unwrap().as_str());
        assert_eq!(n, false);
    }
}
