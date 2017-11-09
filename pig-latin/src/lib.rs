fn apply_vowls_rule(word: &str) -> Option<String> {
    let rules = vec!["a", "o", "e", "i", "u", "xr", "yt"];

    for part in &rules {
        if word.starts_with(part) {
            return Some(word.to_string() + "ay");
        }
    }
    None
}

fn apply_consonant_rule(word: &str) -> String {
    let rules = vec![
        (0, "ch"),
        (1, "qu"),
        (0, "qu"),
        (0, "sch"),
        // Must be before "th"
        (0, "thr"),
        (0, "th"),
        (0, "xr"),
        (0, "yt"),
    ];
    for &(index, part) in &rules {
        if word[index..].starts_with(part) {
            let split = part.len() + index;
            return word[split..].to_string() + &word[..split] + "ay";
        }
    }
    word[1..].to_string() + &word[0..1] + "ay"

}

fn translate_word(word: &str) -> String {
    apply_vowls_rule(word).unwrap_or_else(|| apply_consonant_rule(word))
}

pub fn translate(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| translate_word(word))
        .collect::<Vec<_>>()
        .join(" ")
}
