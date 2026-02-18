use crate::banner::prompt_banner;
use colored::Colorize;

pub fn get_version() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let build_date = env!("DATE");
    let git_head_hash = env!("GIT_HASH");
    println!("{}-{} ({} {})", name, version, git_head_hash, build_date);
}

pub fn get_help() {
    // println!("\nrequire: ");
    // println!("       - rust-diceware binary from crate.io manually installed");
    // println!("");
    prompt_banner();
    println!(
        "\n{}{}",
        "Usage:".bright_green(),
        "\tpaperbackup [OPTIONS]".bright_blue()
    );
    println!("\n{}", "Options: ".bright_green());
    println!(
        "{}{}",
        "\t--set-config".bright_blue(),
        "\t\tSet config & qrcode path".white()
    );
    println!(
        "{}{}",
        "\t--config".bright_blue(),
        "\t\tShow Config".white()
    );
    println!(
        "{}{}",
        "\t--eff".bright_blue(),
        "\t\t\tGenerate Eff random wordlist".white()
    );
    println!(
        "{}{}",
        "\t--eff-lock".bright_blue(),
        "\t\tGenerate paper backup with Eff random wordlist".white()
    );
    println!(
        "{}{}",
        "\t--diceware".bright_blue(),
        "\t\tGenerate passphrase using diceware crate".white()
    );
    println!(
        "{}{}",
        "\t--diceware-lock".bright_blue(),
        "\t\tGenerate qrcoode paper backup with --diceware".white()
    );
    println!(
        "{}{}",
        "\t--mnemonic".bright_blue(),
        "\t\tGenerate passphrase using tiny-bip39 crate".white()
    );
    println!(
        "{}{}",
        "\t--mnemonic-lock".bright_blue(),
        "\t\tGenerate qrcode paper backup using tiny-bip39 crate".white()
    );
    println!(
        "{}{}",
        "\t--unlock".bright_blue(),
        "\t\tUnlock qrcode from directory qrcode/".white()
    );
    println!(
        "{}{}",
        "\t--lock-string".bright_blue(),
        "\t\tGenerate qrcode paper backup from string input".white()
    );
    println!(
        "{}{}",
        "\t--qrcode-no-pgp".bright_blue(),
        "\t\tGenerate qrcode only no pgp".white()
    );
    println!(
        "{}{}",
        "\t--from-file-pgp".bright_blue(),
        "\t\tGenerate qrcode with pgp from file".white()
    );
    println!(
        "{}{}",
        "\t--from-file".bright_blue(),
        "\t\tGenerate qrcode only no pgp from file".white()
    );
    println!(
        "{}{}",
        "\t--convert".bright_blue(),
        "\t\tConvertion string to ?".white()
    );
    println!(
        "{}{}",
        "\t--entropy-check".bright_blue(),
        "\t\tCheck entropy value of password / string".white()
    );
    println!(
        "{}{}",
        "\t--password".bright_blue(),
        "\t\tPassword generator not include Extended ASCII".white()
    );
    println!(
        "{}{}",
        "\t--encode-image ".bright_blue(),
        "\t\tEncode message to image".white()
    );
    println!(
        "{}{}",
        "\t--decode-image ".bright_blue(),
        "\t\tDecode message to image".white()
    );
    println!("{}{}", "\t--version".bright_blue(), "\t\tversion".white());
    println!(
        "{}{}",
        "\t--help".bright_blue(),
        "\t\t\tHelp command!\n".white()
    );
    print!("{}", "Version: ".bright_green());
    get_version();
    println!();
}
