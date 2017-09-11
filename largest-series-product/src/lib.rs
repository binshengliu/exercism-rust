pub fn lsp(s: &str, window: usize) -> Result<u32, String> {
    if window == 0 {
        return Ok(1);
    }

    let (digits, are_all_digits) = s.chars().fold(
        (Vec::new(), true),
        |(mut digits, are_all_digits), c| if c.is_digit(10) {
            digits.push(c.to_digit(10).unwrap());
            (digits, are_all_digits)
        } else {
            (digits, false)
        },
    );

    // Although redundant with ok_or check, it makes the logic simpler
    if digits.len() < window {
        return Err(format!("Input string is too short"))
    }

    if !are_all_digits {
        return Err(format!("Input string contains invalid symbols"));
    }

     digits
        .as_slice()
        .windows(window)
        .map(|w| w.iter().product())
        .max()
        .ok_or(format!("This should never happen"))
}
