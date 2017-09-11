use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let sentence_set: HashSet<_> = sentence.to_lowercase().chars().collect();
    let alphabet: HashSet<_> = (0..26).map(|x| (x + 'a' as u8) as char).collect();

    alphabet.difference(&sentence_set).count() == 0
}
