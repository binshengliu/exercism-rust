use std::fmt::Debug;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Debug + Display>(
    l1: &[T],
    l2: &[T],
) -> Comparison {
    if l1.len() == l2.len() && l1 == l2 {
        return Comparison::Equal;
    }

    if l1.len() < l2.len() && is_sublist(l1, l2) {
        return Comparison::Sublist;
    }

    if l2.len() < l1.len() && is_sublist(l2, l1) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

// Is l1 a sublist of l2
fn is_sublist<T: PartialEq + Debug + Display>(l1: &[T], l2: &[T]) -> bool {
    assert!(l1.len() <= l2.len());

    if l1.is_empty() {
        return true;
    }

    for (l2_index, l2_elem) in
        l2.iter().take(l2.len() - l1.len() + 1).enumerate()
    {
        if l1[0] == *l2_elem && l1 == &l2[l2_index..l2_index + l1.len()] {
            return true;
        }
    }

    false
}
