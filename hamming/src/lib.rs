pub fn hamming_distance(a: &str, b: &str) -> Result<usize, String> {
    if a.len() == b.len() {
        Ok(a.chars().zip(b.chars()).filter(|&(c1, c2)| c1 != c2).count())
    } else {
        Err(format!("The two strings are not of the same length: {} and {}.",
                    a.len(), b.len()))
    }
}
