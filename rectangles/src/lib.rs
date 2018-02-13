extern crate itertools;
use itertools::Itertools;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

pub fn count(input: &[&str]) -> usize {
    let corners: Vec<Vec<Point>> = input
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, ch)| {
                    if ch == '+' {
                        Some(Point { x: row, y: col })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Point>>()
        })
        .collect();

    let mut rect = 0;
    for row_pair in corners.iter().combinations(2) {
        rect += find_rectangles(input, row_pair[0], row_pair[1]);
    }

    return rect;
}

fn find_rectangles(input: &[&str], row1: &[Point], row2: &[Point]) -> usize {
    let mut rect = 0;
    for row1_point_pair1 in row1.iter().combinations(2) {
        for row2_point_pair2 in row2.iter().combinations(2) {
            if row1_point_pair1[0].y == row2_point_pair2[0].y
                && row1_point_pair1[1].y == row2_point_pair2[1].y
            {
                let valid = verify_rectangle(
                    input,
                    row1_point_pair1[0].x,
                    row2_point_pair2[0].x,
                    row1_point_pair1[0].y,
                    row1_point_pair1[1].y,
                );
                if valid {
                    rect += 1;
                }
            }
        }
    }
    return rect;
}

fn verify_rectangle(
    input: &[&str],
    row1: usize,
    row2: usize,
    col1: usize,
    col2: usize,
) -> bool {
    // Check first row, second row, then first col, second col.
    input[row1][col1 + 1..col2]
        .chars()
        .all(|ch| ch == '-' || ch == '+')
        && input[row2][col1 + 1..col2]
            .chars()
            .all(|ch| ch == '-' || ch == '+')
        && input[row1 + 1..row2]
            .iter()
            .map(|line| line.chars().nth(col1).unwrap())
            .all(|ch| ch == '|' || ch == '+')
        && input[row1 + 1..row2]
            .iter()
            .map(|line| line.chars().nth(col2).unwrap())
            .all(|ch| ch == '|' || ch == '+')
}
