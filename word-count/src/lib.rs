use std::collections::HashMap;

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence
        .split(|c| !char::is_alphanumeric(c))
        .filter(|s| !s.is_empty())
        .fold(HashMap::new(), |mut map, s| {
            *map.entry(s.to_lowercase()).or_insert(0) += 1;
            map
        })
}
