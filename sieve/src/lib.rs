pub fn primes_up_to(limit: u32) -> Vec<u32> {
    let mut range: Vec<u32> = (2..limit+1).collect();
    let mut ans: Vec<u32> = Vec::new();
    while !range.is_empty() {
        let first = range[0];
        ans.push(first);
        range = range.into_iter().skip(1).filter(|num| num % first != 0).collect();
    }
    ans
}
