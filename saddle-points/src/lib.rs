use std::collections::HashSet;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut max_pos: HashSet<(usize, usize)> = HashSet::new();
    for (x, row) in input.iter().enumerate() {
        let max = row.iter().max();
        let pos: HashSet<(usize, usize)> = row.iter()
            .cloned()
            .enumerate()
            .filter(|&(_, v)| max.map_or(false, |&max_value| v == max_value))
            .map(|(y, _)| (x, y))
            .collect();
        max_pos.extend(pos);
    }

    let mut min_pos: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..input[0].len() {
        let col: Vec<u64> = input.iter().map(|row| row[y]).collect();
        let min = col.iter().min();
        let pos: HashSet<(usize, usize)> = col.iter()
            .cloned()
            .enumerate()
            .filter(|&(_, v)| min.map_or(false, |&min_value| v == min_value))
            .map(|(x, _)| (x, y))
            .collect();
        min_pos.extend(pos);
    }

    let mut result: Vec<_> = max_pos.intersection(&min_pos).cloned().collect();
    result.sort();
    result
}
