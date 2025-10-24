use std::collections::HashSet;

#[allow(unused)]
use colored::Colorize;
use zxcvbn::zxcvbn;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Zxcvbn {
    guesses: u64,
    guesses_log10: f64,
    crack_times_online_no_throttling_10_per_sec: String,
    crack_times_online_throttling_100_per_sec: String,
    crack_times_offline_slow_hashing_1e4_per_second: String,
    crack_times_offline_fast_hashing_1e10_per_second: String,
    score: String,
}

pub fn zxcvbn_check(check: &str) -> Option<Zxcvbn> {
    if check.is_empty() {
        None
    } else {
        let cal = zxcvbn(check, &[]);
        Some(Zxcvbn {
            guesses: cal.guesses(),
            guesses_log10: cal.guesses_log10(),
            crack_times_online_no_throttling_10_per_sec: cal
                .crack_times()
                .online_no_throttling_10_per_second()
                .to_string(),
            crack_times_online_throttling_100_per_sec: cal
                .crack_times()
                .online_throttling_100_per_hour()
                .to_string(),
            crack_times_offline_slow_hashing_1e4_per_second: cal
                .crack_times()
                .offline_slow_hashing_1e4_per_second()
                .to_string(),
            crack_times_offline_fast_hashing_1e10_per_second: cal
                .crack_times()
                .offline_fast_hashing_1e10_per_second()
                .to_string(),
            score: cal.score().to_string(),
        })
    }
}

#[allow(dead_code)]
pub fn zxcvbn_check_pretty(check: &str) -> Option<String> {
    if check.is_empty() {
        None
    } else {
        // Some("".to_string())
        todo!()
    }
}

pub fn entrophy_calc(pass: &str) -> f64 {
    let pass_length = pass.len() as f64;
    let uniq_char: HashSet<char> = pass.chars().collect();
    let num_char = uniq_char.len() as f64;

    let lowercase_range = check_lowercase(pass);
    let uppercase_range = check_uppercase(pass);
    let numeric_range = check_numeric(pass);
    let punct_range = check_punctuation(pass);
    let ascii_range = check_ascii(pass);

    let num_r = lowercase_range + uppercase_range + numeric_range + punct_range + ascii_range;

    if num_char == 0.0 {
        return 0.0;
    }

    // E = L × log2(R)
    pass_length * (num_r).log2()
}

fn check_ascii(pass: &str) -> f64 {
    if pass.chars().any(|a: char| char::is_ascii(&a)) {
        127.0
    } else {
        0.0
    }
}

fn check_punctuation(pass: &str) -> f64 {
    if pass.chars().any(|a: char| char::is_ascii_punctuation(&a)) {
        33.0
    } else {
        0.0
    }
}

fn check_numeric(pass: &str) -> f64 {
    if pass.chars().any(char::is_numeric) {
        10.0
    } else {
        0.0
    }
}

fn check_lowercase(pass: &str) -> f64 {
    if pass.chars().any(char::is_lowercase) {
        26.0
    } else {
        0.0
    }
}

fn check_uppercase(pass: &str) -> f64 {
    if pass.chars().any(char::is_uppercase) {
        26.0
    } else {
        0.0
    }
}
