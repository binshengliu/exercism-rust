pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }

    let mut ans = Vec::new();
    for i in 2..n+1 {
        if n % i == 0 {
            ans = factors(n / i);
            ans.insert(0, i);
            break;
        }
    }
    ans
}
