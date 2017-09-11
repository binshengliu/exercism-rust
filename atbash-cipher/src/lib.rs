// This is a great solution:
// http://exercism.io/submissions/3294884e643a485ba1c367f370d16745

extern crate itertools;
use itertools::Itertools;

const GROUP_SIZE: usize = 5;

// Only accepts alphabetic and digit
fn encode_char(c: char) -> Option<char> {
    match c {
        'a' ... 'z' => Some(('z' as u8 - (c as u8 - 'a' as u8)) as char),
        '0' ... '9' => Some(c),
        _ => None,
    }
}

pub fn encode(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter_map(encode_char)
        .chunks(GROUP_SIZE)
        .into_iter()
        .map(|mut chunk| chunk.join(""))
        .join(" ")
}

pub fn decode(s: &str) -> String {
    s.chars()
        .filter_map(encode_char)
        .collect::<String>()
}
