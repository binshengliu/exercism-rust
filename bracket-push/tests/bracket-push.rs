extern crate bracket_push;

use bracket_push::*;

#[test]
fn paired_square_brackets() {
    assert!(Brackets::from("[]").are_balanced());
}

#[test]
fn empty_string() {
    assert!(Brackets::from("").are_balanced());
}

#[test]
fn unpaired_brackets() {
    assert!(!Brackets::from("[[").are_balanced());
}

#[test]
fn wrong_ordered_brackets() {
    assert!(!Brackets::from("}{").are_balanced());
}

#[test]
fn wrong_closing_bracket() {
    assert!(!Brackets::from("{]").are_balanced());
}

#[test]
fn paired_with_whitespace() {
    assert!(Brackets::from("{ }").are_balanced());
}

#[test]
fn simple_nested_brackets() {
    assert!(Brackets::from("{[]}").are_balanced());
}

#[test]
fn several_paired_brackets() {
    assert!(Brackets::from("{}[]").are_balanced());
}

#[test]
fn paired_and_nested_brackets() {
    assert!(Brackets::from("([{}({}[])])").are_balanced());
}

#[test]
fn unopened_closing_brackets() {
    assert!(!Brackets::from("{[)][]}").are_balanced());
}

#[test]
fn unpaired_and_nested_brackets() {
    assert!(!Brackets::from("([{])").are_balanced());
}

#[test]
fn paired_and_wrong_nested_brackets() {
    assert!(!Brackets::from("[({]})").are_balanced());
}

#[test]
fn math_expression() {
    assert!(Brackets::from("(((185 + 223.85) * 15) - 543)/2").are_balanced());
}

#[test]
fn complex_latex_expression() {
    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
    assert!(Brackets::from(input).are_balanced());
}
