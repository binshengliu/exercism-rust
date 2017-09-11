use std::collections::HashMap;
use std::collections::HashSet;

const VALID_CHARS: &[char] = &['A', 'T', 'C', 'G'];

pub fn count(c: char, seq: &str) -> Result<usize, String> {
    let valid_chars: HashSet<char> = VALID_CHARS.iter().cloned().collect();
    if !seq.chars().all(|ch| valid_chars.contains(&ch)) {
        return Err(format!("Invalid sequence: {}", seq));
    }

    match c {
        c if valid_chars.contains(&c) => Ok(seq.chars().filter(|&ch| ch == c).count()),
        c => Err(format!("Unknow symbol: {}", c)),
    }
}

pub fn nucleotide_counts(seq: &str) -> Result<HashMap<char, usize>, String> {
    let valid_chars: HashSet<char> = VALID_CHARS.iter().cloned().collect();

    if !seq.chars().all(|ch| valid_chars.contains(&ch)) {
        return Err(format!("Invalid sequence: {}", seq));
    }

    let default: HashMap<char, usize> = VALID_CHARS.iter().map(|&ch| (ch, 0)).collect();
    let result = seq.chars().fold(default, |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    });
    Ok(result)
}
