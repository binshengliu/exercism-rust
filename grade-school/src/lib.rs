use std::collections::HashMap;

pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { roster: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.entry(grade).or_insert(Vec::new()).push(
            student.to_string(),
        );
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = self.roster.keys().cloned().collect::<Vec<u32>>();
        result.sort();
        result
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.roster.get(&grade).cloned().map(|mut v| {
            v.sort();
            v
        })
    }
}
