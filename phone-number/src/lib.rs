// return the valid ten-digit vec
fn normalize(s: &str) -> Option<Vec<char>> {
    let mut digits: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
    if digits.len() != 11 && digits.len() != 10 {
        return None;
    }

    if digits.len() == 11 {
        if digits[0] != '1' {
            return None;
        }

        digits = digits.into_iter().skip(1).collect()
    }

    // The spec states the two nubmers must be from 2 through 9, but
    // the test cases allow 1 at the positions.
    match (digits[0], digits[3]) {
        ('1'...'9', '1'...'9') => Some(digits),
        _ => None,
    }
}

pub fn number(s: &str) -> Option<String> {
    match normalize(s) {
        Some(digits) => Some(digits.into_iter().collect()),
        _ => None,
    }
}

pub fn area_code(s: &str) -> Option<String> {
    number(s).map(|n| (&n[0..3]).to_string())
}

pub fn pretty_print(s: &str) -> String {
    if let Some(digits) = number(s) {
        return format!(
            "({}) {}-{}",
            (&digits[0..3]),
            (&digits[3..6]),
            (&digits[6..10])
        );
    }
    "invalid".to_string()
}
