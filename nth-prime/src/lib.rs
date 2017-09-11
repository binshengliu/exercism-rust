fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    let sqrt_n = ((n as f32).sqrt() as u32) + 1;
    for i in 2..sqrt_n {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

pub fn nth(n: u32) -> Result<u32, &'static str> {
    if n <= 0 {
        return Err("Invalid input");
    }

    let mut num = 2;
    for _ in 1..n {
        num += 1;
        while !is_prime(num) {
            num += 1;
        }
    }

    return Ok(num);
}
