use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
}

pub fn classify(num: u64) -> Result<Classification, &'static str> {
    if num < 1 {
        return Err("Number must be positive");
    }

    let sqrt_num = (num as f64).sqrt() as u64;

    // Time complexity: O(square root of num)
    let mut aliquot_sum = 0;
    aliquot_sum += (2..(sqrt_num + 1)).fold(1, |acc, n| if num % n == 0 {
        acc + n + num / n
    } else {
        acc
    });

    // The square root is duplicated
    if sqrt_num * sqrt_num == num {
        aliquot_sum -= sqrt_num;
    }

    let classification = match aliquot_sum.cmp(&num) {
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
        Ordering::Less => Classification::Deficient,
    };

    Ok(classification)
}
