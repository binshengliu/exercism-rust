use std::collections::HashSet;

pub fn check(s: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    s.chars().all(|c| {
        let c = c.to_lowercase().nth(0).unwrap();
        if !c.is_alphabetic() {
            true
        } else if set.contains(&c) {
            false
        } else {
            set.insert(c);
            true
        }
    })
}
