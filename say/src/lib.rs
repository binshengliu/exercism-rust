//
// See Rust Language Specific Instructions
// below normal exercise description.
//
use std::collections::HashMap;

const BASIC_NUMBERS: &'static [(u64, &'static str)] = &[
    (0, "zero"),
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (10, "ten"),
    (11, "eleven"),
    (12, "twelve"),
    (13, "thirteen"),
    (14, "fourteen"),
    (15, "fifteen"),
    (16, "sixteen"),
    (17, "seventeen"),
    (18, "eighteen"),
    (19, "nineteen"),
    (20, "twenty"),
    (30, "thirty"),
    (40, "forty"),
    (50, "fifty"),
    (60, "sixty"),
    (70, "seventy"),
    (80, "eighty"),
    (90, "ninety"),
];

fn encode_less_than_two_digits(n: u64) -> String {
    let hash: HashMap<u64, &str> = BASIC_NUMBERS.iter().cloned().collect();

    match n {
        0...20 => hash[&n].to_string(),
        tens if tens % 10 == 0 => hash[&tens].to_string(),
        _ => format!("{}-{}", hash[&(n / 10 * 10)], hash[&(n - n / 10 * 10)]),
    }
}

fn encode_less_than_three_digits(n: u64) -> String {
    let hash: HashMap<u64, &str> = BASIC_NUMBERS.iter().cloned().collect();

    match n {
        0...99 => encode_less_than_two_digits(n),
        n if n % 100 == 0 => format!("{} hundred", hash[&(n / 100)]),
        100...999 => {
            format!(
                "{} hundred {}",
                hash[&(n / 100)],
                encode_less_than_two_digits(n - n / 100 * 100)
            )
        }
        _ => panic!("Unexpected three digits: {}", n),
    }
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }

    let mut number = n;
    let v = vec![
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];

    let mut group: Vec<u64> = Vec::new();
    while number >= 1000 {
        group.insert(0, number - number / 1000 * 1000);
        number /= 1000;
    }
    group.insert(0, number);

    // [(1, "million"), (23, "thousand")] format
    let group: Vec<_> = group.iter().zip(v[..group.len()].iter().rev()).collect();

    let group: Vec<_> = group
        .into_iter()
        .filter(|&(x, _)| *x != 0)
        .map(|(&num, scale)| (encode_less_than_three_digits(num), scale))
        .collect();

    for &(ref a, &b) in group.iter() {
        println!("{} {}", a, b);
    }

    group
        .into_iter()
        .fold(
            String::new(),
            |acc, (a, b)| acc + " " + a.as_str() + " " + b,
        )
        .trim()
        .to_string()
}
