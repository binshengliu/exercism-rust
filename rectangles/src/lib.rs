extern crate itertools;
use itertools::Itertools;

pub fn count(input: &[&str]) -> usize {
    let vec: Vec<Vec<usize>> = input
        .iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(
                    |(col, ch)| if ch == '+' { Some(col) } else { None },
                )
                .collect::<Vec<usize>>()
        })
        .collect();

    print!("{:?}", vec);

    let mut rect = 0;
    for line_pair in vec.iter().combinations(2) {
        rect += check_rect(line_pair[0], line_pair[1]);
    }

    return rect;
}

fn check_rect(row1: &[usize], row2: &[usize]) -> usize {
    let mut rect = 0;
    for pair1 in row1.iter().combinations(2) {
        for pair2 in row2.iter().combinations(2) {
            if pair1[0] == pair2[0] && pair1[1] == pair2[1] {
                rect += 1;
            }
        }
    }
    return rect;
}
