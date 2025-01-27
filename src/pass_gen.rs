use passwords::PasswordGenerator;
pub fn gen_password(pass_lenght: usize) -> Option<String> {
    let gen_pass = PasswordGenerator {
        length: pass_lenght,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    if pass_lenght == 0 {
        None
    } else {
        Some(gen_pass.generate_one().unwrap())
    }
}
