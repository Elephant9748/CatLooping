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
            words_string.push_str(word);
            words_string.push('-');
        }

        words_string.pop();

        Some(words_string)
    }
}
