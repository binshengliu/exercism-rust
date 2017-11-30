extern crate itertools;

use itertools::Itertools;

const BOOK_PRICE: f64 = 8.0;
const DISCOUNT: &[f64] = &[1.0, 0.95, 0.9, 0.8, 0.75];

pub fn lowest_price(books: &[usize]) -> f64 {
    if books.is_empty() {
        return 0.0;
    }

    let mut stats = vec![0; 5];
    for &book in books.iter() {
        stats[book - 1] += 1;
    }

    low_price(&stats)
}

pub fn low_price(stats: &[usize]) -> f64 {
    let types = stats.iter().filter(|&&c| c != 0).count();
    if stats.iter().all(|&c| c == 1 || c == 0) {
        return BOOK_PRICE * types as f64 * DISCOUNT[types - 1];
    }

    if stats.iter().filter(|&&c| c != 0).count() == 1 {
        return BOOK_PRICE * stats.iter().sum::<usize>() as f64;
    }

    let mut lowest: Option<f64> = None;
    for count in 2..6 {
        for comb in (1..6).filter(|&c| stats[c - 1] != 0).combinations(count) {
            let mut price = BOOK_PRICE * count as f64 * DISCOUNT[count - 1];
            let mut remain = stats.to_vec();
            for book in comb {
                remain[book - 1] -= 1;
            }
            price += low_price(&remain);
            lowest = lowest.map_or(Some(price), |l| Some(l.min(price)));
        }
    }

    return lowest.unwrap();
}
