#[allow(unused)]
use colored::Colorize;
use zxcvbn::zxcvbn;

#[derive(Debug)]
#[allow(dead_code)]
pub struct EntCheck {
    guesses: u64,
    guesses_log10: f64,
    crack_times_online_no_throttling_10_per_sec: String,
    crack_times_online_throttling_100_per_sec: String,
    crack_times_offline_slow_hashing_1e4_per_second: String,
    crack_times_offline_fast_hashing_1e10_per_second: String,
    score: String,
}

pub fn entropy_check(check: &str) -> Option<EntCheck> {
    if check.is_empty() {
        None
    } else {
        let cal = zxcvbn(check, &[]);
        Some(EntCheck {
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
pub fn entropy_check_pretty(check: &str) -> Option<String> {
    if check.is_empty() {
        None
    } else {
        // Some("".to_string())
        todo!()
    }
}

// todo!() calculate zxcvbn from keepassxc
