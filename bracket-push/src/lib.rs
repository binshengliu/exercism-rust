use std::collections::HashMap;
use std::collections::HashSet;

pub struct Brackets {
    s: String,
    brackets: HashMap<char, char>,
    chars: HashSet<char>,
}

// Modify this array to support more brackets.
const BALANCED_BRACKETS: &[(char, char)] =
    &[('{', '}'), ('[', ']'), ('(', ')')];

impl Brackets {
    pub fn new<'a>(s: &'a str) -> Brackets {
        let brackets = BALANCED_BRACKETS.iter().cloned().collect();

        let chars = BALANCED_BRACKETS
            .iter()
            .flat_map(|&(a, b)| vec![a, b].into_iter())
            .collect();

        Brackets {
            s: s.to_string(),
            brackets: brackets,
            chars: chars,
        }
    }

    pub fn are_balanced(&self) -> bool {
        let mut stack = Vec::new();
        for c in self.s.chars() {
            if !self.chars.contains(&c) {
                continue;
            }

            if self.brackets.contains_key(&c) {
                stack.push(c);
                continue;
            }

            match stack.pop() {
                Some(prev) if self.brackets[&prev] == c => continue,
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

impl<'a> From<&'a str> for Brackets {
    fn from(f: &'a str) -> Self {
        Brackets::new(f)
    }
}
