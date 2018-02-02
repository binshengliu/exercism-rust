use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&'static str], size: usize) -> HashMap<char, usize> {
    if input.is_empty() || size == 0 {
        return HashMap::new();
    }

    let (tx, rx) = mpsc::channel();

    // let mut workers = Vec::new();
    let unit = input.len() / usize::min(input.len(), size);
    for chunk in input.chunks(unit) {
        let sender = mpsc::Sender::clone(&tx);
        let input = chunk.to_vec();
        thread::spawn(move || sender.send(stats(input)).unwrap());
        // workers.push(thread);
    }

    let mut result = HashMap::new();
    for _ in input.chunks(unit) {
        let partial_map = rx.recv().unwrap();
        if result.is_empty() {
            result = partial_map;
        } else {
            for (k, v) in partial_map.into_iter() {
                *result.entry(k).or_insert(0) += v;
            }
        }
    }
    result
}

fn stats(input: Vec<&'static str>) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for astr in input {
        for c in astr.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
            *map.entry(c).or_insert(0) += 1;
        }
    }
    map
}
