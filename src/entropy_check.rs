use zxcvbn::zxcvbn;

#[derive(Debug)]
#[allow(dead_code)]
pub struct EntCheck {
    guesses: u64,
    guesses_log10: f64,
    crack_times: String,
    score: String,
    manual: f64,
}

pub fn entropy_check(check: &str) -> Option<EntCheck> {
    if check.is_empty() {
        None
    } else {
        let cal = zxcvbn(check, &[]);
        Some(EntCheck {
            guesses: cal.guesses(),
            guesses_log10: cal.guesses_log10(),
            crack_times: cal
                .crack_times()
                .online_no_throttling_10_per_second()
                .to_string(),
            score: cal.score().to_string(),
            manual: ((check.len() as f64).log2()) * 12.0,
        })
    }
}
