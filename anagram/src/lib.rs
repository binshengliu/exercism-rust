use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, candidats: &[&'a str]) -> Vec<&'a str> {
    let mut chars: HashMap<char, usize> = HashMap::new();
    for c in word.to_lowercase().chars() {
        *chars.entry(c).or_insert(0) += 1;
    }

    candidats
        .iter()
        .filter(|&&w| w.to_lowercase() != word.to_lowercase())
        .filter_map(|&w| anagram(w, &chars))
        .collect()
}

fn anagram<'a>(word: &'a str, chars: &HashMap<char, usize>) -> Option<&'a str> {
    let mut my_chars: HashMap<char, usize> = HashMap::new();
    for c in word.to_lowercase().chars() {
        *my_chars.entry(c).or_insert(0) += 1;
    }

    if my_chars == *chars { Some(word) } else { None }
}
