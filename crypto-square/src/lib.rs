extern crate itertools;
use itertools::Itertools;

// for filter_map
fn filter_and_convert_to_lowercase(c: char) -> Option<char> {
    if c.is_alphabetic() {
        c.to_lowercase().next()
    } else {
        None
    }
}

pub fn encrypt(text: &str) -> String {
    // Turn into vec of valid chars
    let valid_chars: Vec<char> = text.chars()
        .filter_map(filter_and_convert_to_lowercase)
        .collect();

    // If valid_chars is empty, char_count and row will be zero
    if valid_chars.is_empty() {
        return String::new();
    }

    let char_count = valid_chars.len();
    let row = (char_count as f64).sqrt().floor() as usize;
    let col = (char_count as f64 / row as f64).ceil() as usize;

    let rows: Vec<Vec<char>> =
        valid_chars.chunks(col).map(|c| c.to_vec()).collect();

    (0..col)
        .map(|col_index| {
            (0..row)
                .filter_map(|row_index| rows[row_index].get(col_index))
                .join("")
        })
        .join(" ")
}
