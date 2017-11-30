extern crate itertools;

use std::collections::HashMap;
use itertools::Itertools;

const BOOK_PRICE: f64 = 8.0;
const DISCOUNT: &[f64] = &[1.0, 0.95, 0.9, 0.8, 0.75];

pub fn lowest_price(books: &[usize]) -> f64 {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for &book in books.iter() {
        *map.entry(book).or_insert(0) += 1;
    }

    low_price(&map)
}

pub fn low_price(books: &HashMap<usize, usize>) -> f64 {
    if books.is_empty() {
        return 0.0;
    }

    if books.iter().all(|(_, &v)| v == 1) {
        let count = books.len();
        return BOOK_PRICE * count as f64 * DISCOUNT[count - 1];
    }

    if books.len() == 1 {
        return BOOK_PRICE * *books.values().next().unwrap() as f64;
    }

    let mut lowest: Option<f64> = None;
    for count in 2..6 {
        for comb in books.keys().combinations(count) {
            let mut price = BOOK_PRICE * count as f64 * DISCOUNT[count - 1];
            let mut remain = books.clone();
            for book in comb {
                *remain.get_mut(book).unwrap() -= 1;
                if remain[book] == 0 {
                    remain.remove(book);
                }
            }
            price += low_price(&remain);
            lowest = lowest.map_or(Some(price), |l| Some(l.min(price)));
        }
    }

    return lowest.unwrap();
}
