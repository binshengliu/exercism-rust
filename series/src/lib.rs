pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    let chars: Vec<char> = digits.chars().collect();
    chars
        .windows(len)
        .map(|w| w.iter().collect::<String>())
        .collect()
}
