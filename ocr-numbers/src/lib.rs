extern crate itertools;
#[macro_use]
extern crate lazy_static;
use itertools::Itertools;

lazy_static! {
    static ref OCR_REPRESENTATION: Vec<String> = vec![
        " _ ".to_string() + "| |" + "|_|" + "   ",
        "   ".to_string() + "  |" + "  |" + "   ",
        " _ ".to_string() + " _|" + "|_ " + "   ",
        " _ ".to_string() + " _|" + " _|" + "   ",
        "   ".to_string() + "|_|" + "  |" + "   ",
        " _ ".to_string() + "|_ " + " _|" + "   ",
        " _ ".to_string() + "|_ " + "|_|" + "   ",
        " _ ".to_string() + "  |" + "  |" + "   ",
        " _ ".to_string() + "|_|" + "|_|" + "   ",
        " _ ".to_string() + "|_|" + " _|" + "   ",
    ];
}

pub fn convert(input: &str) -> Result<String, ()> {
    let raw_lines: Vec<&str> = input.split("\n").collect();
    if raw_lines.len() % 4 != 0 {
        return Err(());
    }
    if raw_lines.iter().any(|line| line.len() % 3 != 0) {
        return Err(());
    }

    let mut result: Vec<String> = Vec::new();
    for t in (0..raw_lines.len()).step(4) {
        let mut logical_line: Vec<Vec<char>> = vec![Vec::new(); 4];
        logical_line[0].extend(raw_lines[t + 0].chars());
        logical_line[1].extend(raw_lines[t + 1].chars());
        logical_line[2].extend(raw_lines[t + 2].chars());
        logical_line[3].extend(raw_lines[t + 3].chars());
        let line_result = match_line(logical_line);
        result.push(line_result);
    }

    Ok(result.join(","))
}

fn match_line(logical_line: Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for i in (0..logical_line[0].len()).step(3) {
        let mut digit: Vec<char> = Vec::new();
        digit.extend(&logical_line[0][i..i + 3]);
        digit.extend(&logical_line[1][i..i + 3]);
        digit.extend(&logical_line[2][i..i + 3]);
        digit.extend(&logical_line[3][i..i + 3]);
        let digit = match_digit(&digit.into_iter().collect::<String>());
        result += &digit;
    }
    result
}

fn match_digit(digit: &str) -> String {
    OCR_REPRESENTATION
        .iter()
        .position(|x| x == digit)
        .map(|index| index.to_string())
        .unwrap_or("?".to_string())
}
