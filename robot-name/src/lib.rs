extern crate rand;
#[macro_use]
extern crate lazy_static;

use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

// thread_local is not appropriate here, for in a multi-threading
// environment, thread_local may also cause name conflicts.
lazy_static! {
    static ref USED: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

const MAX_TRIALS: usize = 1000;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: unique_random_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = unique_random_name();
    }
}

fn unique_random_name() -> String {
    for _ in 0..MAX_TRIALS {
        let name = random_name();
        let mut used = USED.lock().unwrap();
        if !used.contains(&name) {
            used.insert(name.clone());
            return name;
        }
    }

    panic!("Unable to find an unused name after {} trials.", MAX_TRIALS);
}

fn random_name() -> String {
    let mut rng1 = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    (0..2)
        .map(|_| rng1.gen_range(0, 26))
        .map(|c| c + 'A' as u8)
        .chain((0..3).map(|_| rng2.gen_range(0, 10) + '0' as u8))
        .map(|c| c as char)
        .collect()
}
