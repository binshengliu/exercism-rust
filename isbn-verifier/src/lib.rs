pub fn to_digit(c: &char) -> u32 {
    if c.is_digit(10) {
        *c as u32 - ('0' as u32 - 0)
    } else {
        assert_eq!(*c, 'X');
        10
    }
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: Vec<char> = isbn.chars()
        .filter(|&c| c.is_digit(10) || c == 'X')
        .collect();

    if isbn.len() != 10 {
        return false;
    }

    if isbn[..9].iter().any(|c| !c.is_digit(10)) {
        return false;
    }

    let digits: Vec<u32> = isbn.iter().map(|c| to_digit(c)).collect();

    let sum: u32 = digits.iter().zip((1..11).rev()).map(|(a, b)| a * b).sum();
    sum % 11 == 0
}
