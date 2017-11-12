extern crate itertools;
use itertools::Itertools;
use Category::*;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(input: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands: Vec<_> = input.iter().map(|s| Hand::new(s)).collect();
    hands.sort_by(|h1, h2| h2.category.cmp(&h1.category));
    let result = hands
        .iter()
        .take_while(|&h| &h.category == &hands[0].category)
        .map(|h| h.input)
        .collect();
    Some(result)
}

#[derive(Copy, Hash, Eq, PartialEq, Clone)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

// Do not derive Eq because we don't compare suit for a card.
#[derive(Clone)]
struct Card {
    value: u8,
    suit: Suit,
}

impl<'a> From<&'a str> for Card {
    fn from(s: &str) -> Self {
        let value = match &s[..s.len() - 1] {
            "A" => 14,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            c => c.parse::<u8>().unwrap(),
        };
        assert!(value >= 2 && value <= 14);

        let suit = match &s[s.len() - 1..] {
            "C" => Suit::Club,
            "D" => Suit::Diamond,
            "H" => Suit::Heart,
            "S" => Suit::Spade,
            _ => unreachable!(),
        };
        Card { value, suit }
    }
}

#[derive(PartialOrd, Ord, Debug, Clone, PartialEq, Eq)]
enum Category {
    HighCard(Vec<u8>),
    OnePair(Vec<u8>),
    TwoPair(Vec<u8>),
    ThreeOfAKind(Vec<u8>),
    Straight(u8),
    Flush(Vec<u8>),
    FullHouse(Vec<u8>),
    FourOfAKind(Vec<u8>),
    StraightFlush(u8),
}

use std::collections::HashMap;
impl Category {
    pub fn new(cards: &[Card]) -> Category {
        let mut value_map: HashMap<u8, usize> = HashMap::new();
        let mut suit_map: HashMap<Suit, usize> = HashMap::new();
        for card in cards.iter() {
            *value_map.entry(card.value).or_insert(0) += 1;
            *suit_map.entry(card.suit).or_insert(0) += 1;
        }
        let values = value_map.into_iter().sorted_by(|&(k1, v1), &(k2, v2)| {
            v2.cmp(&v1).then(k2.cmp(&k1))
        });
        let suits = suit_map.into_iter().sorted_by(
            |&(_, v1), &(_, v2)| v2.cmp(&v1),
        );
        straight_flush(&values, &suits)
            .or(four_of_a_kind(&values))
            .or(full_house(&values))
            .or(flush(&values, &suits))
            .or(straight(&values))
            .or(three_of_a_kind(&values))
            .or(two_pair(&values))
            .or(one_pair(&values))
            .unwrap_or(high_card(&values))
    }
}

struct Hand<'a> {
    // The cards are always sorted
    input: &'a str,
    category: Category,
}

impl<'a> Hand<'a> {
    pub fn new(input: &str) -> Hand {
        let cards: Vec<_> =
            input.split_whitespace().map(|s| Card::from(s)).collect();
        let category = Category::new(&cards);
        Hand { input, category }
    }
}

fn straight_flush(
    values: &Vec<(u8, usize)>,
    suits: &Vec<(Suit, usize)>,
) -> Option<Category> {
    flush(values, suits).and_then(|_| {
        straight(values).and_then(|category| match category {
            Straight(value) => Some(StraightFlush(value)),
            _ => panic!("The type must be Straight"),
        })
    })
}

fn four_of_a_kind(values: &Vec<(u8, usize)>) -> Option<Category> {
    if values.iter().map(|v| v.1).collect_vec() == [4, 1] {
        Some(FourOfAKind(values.into_iter().map(|v| v.0).collect()))
    } else {
        None
    }
}

fn full_house(values: &Vec<(u8, usize)>) -> Option<Category> {
    if values.iter().map(|v| v.1).collect_vec() == [3, 2] {
        Some(FullHouse(values.into_iter().map(|v| v.0).collect()))
    } else {
        None
    }
}

fn flush(
    values: &Vec<(u8, usize)>,
    suits: &Vec<(Suit, usize)>,
) -> Option<Category> {
    if suits.len() == 1 {
        Some(Flush(values.into_iter().map(|v| v.0).collect()))
    } else {
        None
    }
}

fn straight(values: &Vec<(u8, usize)>) -> Option<Category> {
    let lowest = values.last().unwrap().0;
    if values.iter().rev().enumerate().skip(1).all(|(i, c)| {
        lowest + i as u8 == c.0
    })
    {
        Some(Straight(values[0].0))
    } else if values.iter().map(|c| c.0).collect_vec() == &[14, 5, 4, 3, 2] {
        Some(Straight(5))
    } else {
        None
    }
}

fn three_of_a_kind(values: &Vec<(u8, usize)>) -> Option<Category> {
    if values.iter().map(|v| v.1).collect_vec() == [3, 1, 1] {
        Some(ThreeOfAKind(values.into_iter().map(|v| v.0).collect()))
    } else {
        None
    }
}

fn two_pair(values: &Vec<(u8, usize)>) -> Option<Category> {
    if values.iter().map(|v| v.1).collect_vec() == [2, 2, 1] {
        Some(TwoPair(values.into_iter().map(|v| v.0).collect()))
    } else {
        None
    }
}

fn one_pair(values: &Vec<(u8, usize)>) -> Option<Category> {
    if values.iter().map(|v| v.1).collect_vec() == [2, 1, 1, 1] {
        Some(OnePair(values.into_iter().map(|v| v.0).collect()))
    } else {
        None
    }
}

fn high_card(values: &Vec<(u8, usize)>) -> Category {
    HighCard(values.into_iter().map(|v| v.0).collect())
}
