#![feature(conservative_impl_trait)]

pub type Domino = (usize, usize);

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    let begin = input[0].0;
    let end = input[0].0;

    chain_begin_end(begin, end, input)
}

fn chain_begin_end(
    begin: usize,
    end: usize,
    input: &[Domino],
) -> Option<Vec<Domino>> {
    if input.len() == 1 {
        let domino = input[0];
        if domino.0 == begin && domino.1 == end {
            return Some(vec![domino]);
        } else if domino.1 == begin && domino.0 == end {
            return Some(vec![(domino.1, domino.0)]);
        } else {
            return None;
        }
    }

    for (index, candidate) in iter_candidates(begin, input) {
        let new: Vec<Domino> = input[..index]
            .iter()
            .chain(&input[index + 1..])
            .cloned()
            .collect();

        if let Some(mut chain) = chain_begin_end(candidate.1, end, &new) {
            chain.insert(0, candidate);
            return Some(chain);
        }
    }

    return None;
}

fn iter_candidates<'a>(
    begin: usize,
    input: &'a [Domino],
) -> impl Iterator<Item = (usize, Domino)> + 'a {
    input
        .iter()
        .cloned()
        .enumerate()
        .filter(move |&(_, d)| d.0 == begin || d.1 == begin)
        .map(move |(index, d)| if d.0 == begin {
            (index, d)
        } else {
            (index, (d.1, d.0))
        })
}
