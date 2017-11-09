extern crate pig_latin as pl;

#[test]
fn test_word_beginning_with_a() {
    assert_eq!(pl::translate("apple"), "appleay");
}

#[test]
fn test_word_beginning_with_e() {
    assert_eq!(pl::translate("ear"), "earay");
}

#[test]
fn test_word_beginning_with_i() {
    assert_eq!(pl::translate("igloo"), "iglooay");
}

#[test]
fn test_word_beginning_with_o() {
    assert_eq!(pl::translate("object"), "objectay");
}

#[test]
fn test_word_beginning_with_u() {
    assert_eq!(pl::translate("under"), "underay");
}

#[test]
fn test_word_beginning_with_a_vowel_and_followed_by_a_qu() {
    assert_eq!(pl::translate("equal"), "equalay");
}

#[test]
fn test_word_beginning_with_p() {
    assert_eq!(pl::translate("pig"), "igpay");
}

#[test]
fn test_word_beginning_with_k() {
    assert_eq!(pl::translate("koala"), "oalakay");
}

#[test]
fn test_word_beginning_with_y() {
    assert_eq!(pl::translate("yellow"), "ellowyay");
}

#[test]
fn test_word_beginning_with_x() {
    assert_eq!(pl::translate("xenon"), "enonxay");
}

#[test]
fn test_word_beginning_with_q_without_a_following_u() {
    assert_eq!(pl::translate("qat"), "atqay");
}

#[test]
fn test_word_beginning_with_ch() {
    assert_eq!(pl::translate("chair"), "airchay");
}

#[test]
fn test_word_beginning_with_qu() {
    assert_eq!(pl::translate("queen"), "eenquay");
}

#[test]
fn test_word_beginning_with_qu_and_a_preceding_consonant() {
    assert_eq!(pl::translate("square"), "aresquay");
}

#[test]
fn test_word_beginning_with_th() {
    assert_eq!(pl::translate("therapy"), "erapythay");
}

#[test]
fn test_word_beginning_with_thr() {
    assert_eq!(pl::translate("thrush"), "ushthray");
}

#[test]
fn test_word_beginning_with_sch() {
    assert_eq!(pl::translate("school"), "oolschay");
}

#[test]
fn test_word_beginning_with_yt() {
    assert_eq!(pl::translate("yttria"), "yttriaay");
}

#[test]
fn test_word_beginning_with_xr() {
    assert_eq!(pl::translate("xray"), "xrayay");
}

#[test]
fn test_a_whole_phrase() {
    assert_eq!(pl::translate("quick fast run"), "ickquay astfay unray");
}
