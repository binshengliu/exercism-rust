pub fn raindrops(num: u32) -> String {
    let mut str = String::new();

    if num % 3 == 0 {
        str.push_str("Pling")
    }

    if num % 5 == 0 {
        str.push_str("Plang")
    }

    if num % 7 == 0 {
        str.push_str("Plong")
    }

    if str.is_empty() {
        str = num.to_string();
    }

    return str;
}
