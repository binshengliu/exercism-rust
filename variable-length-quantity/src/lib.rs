/// Encode one number
fn convert_number(n: u32) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut v = n;
    let mut result: Vec<u8> = Vec::new();
    while v != 0 {
        let current = (v % 128) as u8;
        if result.is_empty() {
            result.insert(0, current);
        } else {
            result.insert(0, current | 0x80);
        }
        v = v / 128;
    }

    result
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|&v| convert_number(v))
        .collect::<Vec<u8>>()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    use std::u32;
    if bytes[bytes.len() - 1] & 0x80 != 0 {
        return Err("Invalid input");
    }

    let mut n: u32 = 0;
    bytes
        .iter()
        .filter_map(|b| {
            let (value, overflow) = n.overflowing_mul(128);
            if overflow {
                return Some(Err("Overflow"));
            }

            let (value, overflow) = value.overflowing_add((b & 0x7f) as u32);
            if overflow {
                return Some(Err("Overflow"));
            }

            if b & 0x80 != 0 {
                n = value;
                None
            } else {
                n = 0;
                Some(Ok(value))
            }
        })
        .collect()
}
