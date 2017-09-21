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

// Dp approach. Is l1 a sublist of l2
fn is_sublist<T: PartialEq + Debug + Display>(l1: &[T], l2: &[T]) -> bool {
    if l1.is_empty() {
        return true;
    }

    let len_l1 = l1.len();
    let len_l2 = l2.len();

    let mut dp = Vec::<usize>::with_capacity(len_l2 + 1);
    dp.resize(len_l2 + 1, 0);

    for row in 1..(len_l1 + 1) {
        let mut new_dp = Vec::<usize>::with_capacity(len_l2 + 1);
        new_dp.resize(len_l2 + 1, 0);
        let mut has_equal_in_this_row = false;
        for col in 1..(len_l2 + 1) {
            if len_l2 - col < len_l1 - row && !has_equal_in_this_row {
                return false;
            }

            if l1[row - 1] == l2[col - 1] {
                has_equal_in_this_row = true;
                new_dp[col] = dp[col - 1] + 1;
            }

            if new_dp[col] == len_l1 {
                return true;
            }
        }

        if !has_equal_in_this_row {
            return false;
        }

        dp = new_dp;
    }


    false
}
