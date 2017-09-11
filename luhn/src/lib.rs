use std::collections::HashSet;

pub fn is_valid(s: &str) -> bool {
    let allowed_chars: HashSet<char> = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ']
        .iter()
        .cloned()
        .collect();

    if !s.chars().all(|x| allowed_chars.contains(&x)) {
        return false;
    }

    if s.chars().filter(|x| x.is_digit(10)).count() <= 1 {
        return false;
    }

    let sum = s.chars()
        .filter_map(|x| x.to_digit(10))
        .rev()
        .enumerate()
        .fold(0, |acc, (index, x)| match index % 2 == 0 {
            true => acc + x,
            false if x * 2 > 9 => acc + x * 2 - 9,
            false => acc + x * 2,
        });

    sum % 10 == 0
}
