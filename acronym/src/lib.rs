// Extract a abbreviated string from a single word. Mainly handle
// ALL-UPPERCASE and CamelCase.
fn abbreviate_word(word: &str) -> String {
    // Ignore non-alphabetic. If the char is an alphabet and is uppercase.
    if word.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) {
        word.chars().nth(0).unwrap().to_string()
    } else {
        word.chars()
            .enumerate()
            .filter_map(|(i, c)| if i == 0 || c.is_uppercase() {
                Some(c.to_uppercase().nth(0).unwrap())
            } else {
                None
            })
            .collect()
    }
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_whitespace()     // split by whitespace first
        .map(|word: &str| {
            word.split('-')     // split by hyphen
                .map(|dehyphened_word: &str| {
                    abbreviate_word(dehyphened_word)
                })
                .collect::<String>()
        })
        .collect()
}
