extern crate meval;

use std::collections::HashMap;

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

    fn try(&self, digits: &Vec<char>) -> Option<HashMap<char, u8>> {
        let map: HashMap<char, char> = self.letters
            .iter()
            .cloned()
            .zip(digits.iter().cloned())
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

    pub fn iterate_permutations(
        &self,
        digits: Vec<char>,
        n: usize,
        perm: Vec<char>,
    ) -> Option<HashMap<char, u8>> {
        if n == 0 {
            return self.try(&perm);
        }
        for (i, &c) in digits.iter().enumerate() {
            let subset: Vec<char> = digits[0..i]
                .iter()
                .chain(digits[i + 1..].iter())
                .cloned()
                .collect();
            let result = self.iterate_permutations(
                subset,
                n - 1,
                perm.iter()
                    .chain(vec![c].iter())
                    .cloned()
                    .collect::<Vec<_>>(),
            );
            if result.is_some() {
                return result;
            }
        }
        None
    }

    pub fn solve(&self) -> Option<HashMap<char, u8>> {
        let digits: Vec<char> =
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        self.iterate_permutations(digits, self.letters.len(), Vec::new())
    }
}
