extern crate acronym;

#[test]
fn basic() {
  assert_eq!(acronym::abbreviate("Portable Network Graphics"), "PNG");
}

#[test]
fn lowercase_words() {
  assert_eq!(acronym::abbreviate("Ruby on Rails"), "ROR");
}

#[test]
fn camelcase() {
    assert_eq!(acronym::abbreviate("HyperText Markup Language"), "HTML");
}

#[test]
fn punctuation() {
    assert_eq!(acronym::abbreviate("First In, First Out"), "FIFO");
}

#[test]
fn all_caps_words() {
    assert_eq!(acronym::abbreviate("PHP: Hypertext Preprocessor"), "PHP");
}

#[test]
fn non_acronym_all_caps_word() {
    assert_eq!(acronym::abbreviate("GNU Image Manipulation Program"), "GIMP");
}

#[test]
fn hyphenated() {
    assert_eq!(acronym::abbreviate("Complementary metal-oxide semiconductor"),
               "CMOS");
}
