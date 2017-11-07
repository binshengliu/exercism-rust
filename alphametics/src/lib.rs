extern crate meval;
extern crate itertools;
extern crate permutohedron;

use std::collections::HashMap;
use itertools::Itertools;
use permutohedron::Heap;

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let solver = Solver::new(puzzle);
    solver.solve()
}

struct Solver {
    first_letters: Vec<char>,
    letters: Vec<char>,
    left: String,
    right: String,
}

impl Solver {
    pub fn new(puzzle: &str) -> Solver {
        let mut iter = puzzle.split("==");
        let left = iter.next().unwrap().trim().to_string();
        let right = iter.next().unwrap().trim().to_string();

        let mut first_letters: Vec<char> = puzzle
            .split(|c: char| !c.is_alphabetic())
            .filter(|puzzle| !puzzle.is_empty())
            .map(|s| s.chars().nth(0).unwrap())
            .collect();
        first_letters.sort();
        first_letters.dedup();

        let mut letters: Vec<char> =
            puzzle.chars().filter(|c| c.is_alphabetic()).collect();
        letters.sort();
        letters.dedup();

        Solver {
            first_letters,
            letters,
            left,
            right,
        }
    }

    fn try(&self, digits: &Vec<&char>) -> Option<HashMap<char, u8>> {
        let map: HashMap<char, char> = self.letters
            .iter()
            .cloned()
            .zip(digits.iter().cloned().cloned())
            .collect();

        let leading_zero = map.iter().any(|(key, &value)| {
            self.first_letters.contains(key) && value == '0'
        });

        if leading_zero {
            return None;
        }

        let left: String = self.left
            .chars()
            .map(|c| if c.is_alphabetic() { map[&c] } else { c })
            .collect();
        let right: String = self.right
            .chars()
            .map(|c| if c.is_alphabetic() { map[&c] } else { c })
            .collect();

        let result = meval::eval_str(left).unwrap();

        if result.to_string() == right {
            Some(
                map.into_iter()
                    .map(|(key, value)| (key, value as u8 - '0' as u8))
                    .collect::<HashMap<char, u8>>(),
            )
        } else {
            None
        }
    }

    pub fn solve(&self) -> Option<HashMap<char, u8>> {
        let digits: Vec<char> =
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for mut combination in digits.iter().combinations(self.letters.len()) {
            let heap = Heap::new(&mut combination);
            for permutation in heap {
                let result = self.try(&permutation);
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }
}
