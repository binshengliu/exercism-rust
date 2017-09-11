pub fn encode(msg: &str) -> String {
    if msg.is_empty() {
        return String::new();
    }

    let s = msg.to_string();
    let mut ans = String::new();
    let mut iter = s.chars();
    let mut prev = iter.next().unwrap();
    let mut prev_count = 1;
    for c in iter {
        if c == prev {
            prev_count += 1;
            continue;
        }

        if prev_count > 1 {
            ans += &prev_count.to_string();
        }
        ans += &prev.to_string();
        prev = c;
        prev_count = 1;
    }

    if prev_count > 1 {
        ans += &prev_count.to_string();
    }
    ans += &prev.to_string();

    ans
}

pub fn decode(msg: &str) -> String {
    if msg.is_empty() {
        return String::new();
    }

    let s = msg.to_string();
    let mut iter = s.chars();
    let mut ans = String::new();
    while let Some(mut c) = iter.next() {
        if c.is_alphabetic() || c.is_whitespace() {
            ans += &c.to_string();
            continue;
        }

        let mut num = String::new();
        // When the loop ends, c is the value of the next char
        while c.is_numeric() {
            num += &c.to_string();
            c = iter.next().unwrap();
        }
        let count: u32 = num.parse().unwrap();
        ans += &(0..count).map(|_| c).collect::<String>();
    }
    ans
}
