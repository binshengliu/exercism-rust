pub fn sum_of_multiples(n: u32, v: &Vec<u32>) -> u32 {
    (1..n).filter(|x| v.iter().any(|i| x % i == 0)).sum()
}
