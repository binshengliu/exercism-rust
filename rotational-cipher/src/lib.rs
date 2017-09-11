pub fn rotate(msg: &str, num: u8) -> String {
    msg.chars()
        .map(|c| if c.is_uppercase() {
            ((c as u8 - 'A' as u8 + num) % 26 + 'A' as u8) as char
        } else if c.is_lowercase() {
            ((c as u8 - 'a' as u8 + num) % 26 + 'a' as u8) as char
        } else {
            c
        })
        .collect::<String>()
}
