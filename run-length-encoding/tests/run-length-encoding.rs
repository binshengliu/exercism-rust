extern crate run_length_encoding as rle;

// encoding tests

#[test]
fn test_encode_empty_string() {
    assert_eq!("", rle::encode(""));
}

#[test]
fn test_encode_single_characters() {
    assert_eq!("XYZ", rle::encode("XYZ"));
}

#[test]
fn test_encode_string_with_no_single_characters() {
    assert_eq!("2A3B4C", rle::encode("AABBBCCCC"));
}

#[test]
fn test_encode_single_characters_mixed_with_repeated_characters() {
    assert_eq!("12WB12W3B24WB", rle::encode(
        "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB"));
}

#[test]
fn test_encode_multiple_whitespace_mixed_in_string() {
    assert_eq!("2 hs2q q2w2 ", rle::encode("  hsqq qww  "));
}

#[test]
fn test_encode_lowercase_characters() {
    assert_eq!("2a3b4c", rle::encode("aabbbcccc"));
}

// decoding tests

#[test]
fn test_decode_empty_string() {
    assert_eq!("", rle::decode(""));
}

#[test]
fn test_decode_single_characters_only() {
    assert_eq!("XYZ", rle::decode("XYZ"));
}

#[test]
fn test_decode_string_with_no_single_characters() {
    assert_eq!("AABBBCCCC", rle::decode("2A3B4C"));
}

#[test]
fn test_decode_single_characters_with_repeated_characters() {
    assert_eq!("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB",
               rle::decode("12WB12W3B24WB"));
}

#[test]
fn test_decode_multiple_whitespace_mixed_in_string() {
    assert_eq!("  hsqq qww  ", rle::decode("2 hs2q q2w2 "));
}

#[test]
fn test_decode_lower_case_string() {
    assert_eq!("aabbbcccc", rle::decode("2a3b4c"));
}

// consistency test

#[test]
fn test_consistency() {
    assert_eq!("zzz ZZ  zZ", rle::decode(rle::encode("zzz ZZ  zZ").as_str()));
}
