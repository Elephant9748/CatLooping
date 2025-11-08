use std::{
    env::{self},
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{banner::prompt_banner, env_paper::ENV_CONFIG, paper_backup::catch_stdin};

#[allow(dead_code)]
enum Valid {
    Num(String),
    Path(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Configs {
    pub config: Config,
    pub qrcode: Qrcode,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Config {
    pub path: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Qrcode {
    pub path: String,
}

pub fn set_config() {
    prompt_banner();
    let config_default_path = "$HOME/.config/paperbackup";

    let homedir =
        set_config_path(config_default_path.to_owned()).expect("--> failed check home dir");
    let forcedir = force_create_dir_check_with_file(homedir.to_string());
    let mut config_path = String::from("config.toml");
    if forcedir.is_empty() {
        config_path = homedir + "/config.toml";
    }

    print!(
        "\n{}: ",
        ":: where to saved qrcode (default \"$HOME/.local/share/paperbackup/qrcode/\")"
            .bright_green()
    );
    let stdin_qrcode = catch_stdin();
    let mut qrcode_path = String::from("");
    let homedir = set_qrcode_path(stdin_qrcode.to_owned()).expect("--> failed check home dir");
    let forcedir = force_create_dir_check_with_file(homedir.to_string());
    if forcedir.is_empty() {
        qrcode_path = homedir;
    }

    let config = Configs {
        config: Config {
            path: config_path.to_owned(),
        },
        qrcode: Qrcode { path: qrcode_path },
    };

    // write file
    let serdetoml = toml::to_string(&config).expect("--> toml to string failed");
    let file = File::create(&config_path).expect(":: config.toml not found");
    let mut buf_writer = BufWriter::new(file);
    let _ = buf_writer.write_all(serdetoml.as_bytes());
    let _ = buf_writer.flush();

    init_done(config);
}

pub fn show_config() {
    let readenv = env::var(ENV_CONFIG).expect("--> Failed to read shells env");
    let readtoml = read_config_file(&readenv).expect("--> Failed to read toml file");
    println!("\x1b[32m{:#?}", readtoml);
}

fn init_done(config: Configs) {
    println!(
        "\x1b[33m{:#?}\n{}{}",
        config,
        "::".bright_blue(),
        " Write config file succeed.".bright_green()
    );
    println!(
        "\n{}",
        "Please set your environment variable".bright_green()
    );
    println!("{}", "Bash edit '~/.bashrc' put:".bright_yellow());
    println!("_____________________________________");
    println!("  \"export {}={}\"", ENV_CONFIG, config.config.path);
    println!("_____________________________________");
    println!(
        "{}",
        "fish edit '~/config/config.fish' put:".bright_yellow()
    );
    println!("_____________________________________");
    println!("if status --is-interactive");
    println!("  # ...");
    println!("  # ...");
    println!("  \"set -x {} {}\"", ENV_CONFIG, config.config.path);
    println!("end");
    println!("_____________________________________");
}

fn check_valid(val: Valid) -> bool {
    match val {
        Valid::Num(a) => a.as_str().chars().all(|c| c.is_numeric()),
        Valid::Path(a) => Path::new(&a).exists(),
    }
}

fn force_create_dir_check_with_file(b: String) -> String {
    let mut toml_file = String::from("");
    if !Path::new(&b).exists() {
        toml_file = b.replace("config.toml", "");
        std::fs::create_dir_all(&toml_file).expect(":: force_create_dir(b) failed");
        return toml_file;
    }
    toml_file
}

fn force_create_dir(b: String) -> String {
    if !Path::new(&b).exists() {
        std::fs::create_dir_all(&b).expect(":: force_create_dir(b) failed");
    }
    b
}

fn set_config_path(p: String) -> Result<String, String> {
    if p.starts_with("~") {
        let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
        let mut full_home_dir = PathBuf::from(home_dir);
        full_home_dir.push(&p[2..]);
        if check_valid(Valid::Path(full_home_dir.display().to_string())) {
            Ok(full_home_dir.display().to_string())
        } else {
            let path = force_create_dir(full_home_dir.display().to_string());
            Ok(path)
        }
    } else if p.starts_with("$HOME") {
        let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
        let mut full_home_dir = PathBuf::from(home_dir);
        full_home_dir.push(&p[6..]);
        if check_valid(Valid::Path(full_home_dir.display().to_string())) {
            Ok(full_home_dir.display().to_string())
        } else {
            let path = force_create_dir(full_home_dir.display().to_string());
            Ok(path)
        }
    } else if p.is_empty() {
        let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
        let mut default_path = PathBuf::from(home_dir);
        default_path.push(".config/paperbackup");
        let path = force_create_dir(default_path.display().to_string());
        Ok(path)
    } else {
        let path = force_create_dir(p);
        Ok(path)
    }
}

pub fn set_qrcode_path(p: String) -> Result<String, String> {
    if p.starts_with("~") {
        let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
        let mut full_home_dir = PathBuf::from(home_dir);
        full_home_dir.push(&p[2..]);
        if check_valid(Valid::Path(full_home_dir.display().to_string())) {
            Ok(full_home_dir.display().to_string())
        } else {
            let path = force_create_dir(full_home_dir.display().to_string());
            Ok(path)
        }
    } else if p.starts_with("$HOME") {
        let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
        let mut full_home_dir = PathBuf::from(home_dir);
        full_home_dir.push(&p[6..]);
        if check_valid(Valid::Path(full_home_dir.display().to_string())) {
            Ok(full_home_dir.display().to_string())
        } else {
            let path = force_create_dir(full_home_dir.display().to_string());
            Ok(path)
        }
    } else if p.is_empty() {
        let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
        let mut default_path = PathBuf::from(home_dir);
        default_path.push(".local/share/paperbackup/qrcode");
        let path = force_create_dir(default_path.display().to_string());
        Ok(path)
    } else {
        let path = force_create_dir(p);
        Ok(path)
    }
}

pub fn read_config_file(filepath: &str) -> Result<Configs, String> {
    let file = File::open(filepath).expect("--> config.toml file not found");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("--> Failed to read buf_reader read_config_file fn");
    let config_toml: Configs = toml::from_str(&contents).unwrap();
    if contents.is_empty() {
        return Err("--> contents buf_reader empty read_config_file".to_string());
    }
    Ok(config_toml)
}
