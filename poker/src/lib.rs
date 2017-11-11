extern crate itertools;
use itertools::Itertools;
use std::cmp::Ordering;
use Category::*;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(input: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands: Vec<_> = input.iter().map(|s| Hand::new(s)).collect();
    hands.sort();
    let result = hands
        .iter()
        .rev()
        .take_while(|&h| Some(h) == hands.last())
        .map(|h| h.cards)
        .collect();
    Some(result)
}

#[derive(PartialEq, Clone)]
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

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(PartialOrd, Ord, Debug, Clone, PartialEq, Eq)]
enum Category {
    HighCard { kickers: Vec<u8> },
    OnePair { pair: u8, kickers: Vec<u8> },
    TwoPair { high: u8, low: u8, kicker: u8 },
    ThreeOfAKind { triplet: u8, kickers: Vec<u8> },
    Straight { highest: u8 },
    Flush { kickers: Vec<u8> },
    FullHouse { triplet: u8, pair: u8 },
    FourOfAKind { quad: u8, kicker: u8 },
    StraightFlush { highest: u8 },
}

impl Category {
    pub fn new(cards: &[Card]) -> Category {
        straight_flush(cards)
            .or(four_of_a_kind(cards))
            .or(full_house(cards))
            .or(flush(cards))
            .or(straight(cards))
            .or(three_of_a_kind(cards))
            .or(two_pair(cards))
            .or(one_pair(cards))
            .unwrap_or(high_card(cards))
    }
}

struct Hand<'a> {
    // The cards are always sorted
    cards: &'a str,
    category: Category,
}

impl<'a> Hand<'a> {
    pub fn new(input: &str) -> Hand {
        let cards: Vec<_> =
            input.split_whitespace().map(|s| Card::from(s)).collect();
        let category = Category::new(&cards);
        Hand {
            cards: input,
            category: category,
        }
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Hand) -> bool {
        self.category == other.category
    }
}

impl<'a> Eq for Hand<'a> {}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.category.cmp(&other.category)
    }
}

fn straight_flush(cards: &[Card]) -> Option<Category> {
    flush(cards).and_then(|_| {
        straight(cards).and_then(|category| match category {
            Straight { highest } => Some(StraightFlush { highest: highest }),
            _ => panic!("The type must be Straight"),
        })
    })
}

fn four_of_a_kind(cards: &[Card]) -> Option<Category> {
    let cards = cards.iter().sorted();
    let (quad, kicker) = if cards[0].value == cards[3].value {
        (cards[0].value, cards[4].value)
    } else if cards[1].value == cards[4].value {
        (cards[1].value, cards[0].value)
    } else {
        return None;
    };

    Some(FourOfAKind { quad, kicker })
}

fn full_house(cards: &[Card]) -> Option<Category> {
    let cards = cards.iter().sorted();
    let (triplet, pair) = if cards[0].value == cards[2].value &&
        cards[3].value == cards[4].value
    {
        (cards[0].value, cards[3].value)
    } else if cards[0].value == cards[1].value &&
               cards[2].value == cards[4].value
    {
        (cards[2].value, cards[0].value)
    } else {
        return None;
    };
    Some(FullHouse { triplet, pair })
}

fn flush(cards: &[Card]) -> Option<Category> {
    if cards[1..].iter().all(|c| cards[0].suit == c.suit) {
        let cards = cards.iter().sorted_by(|c1, c2| c2.value.cmp(&c1.value));
        Some(Flush { kickers: cards.iter().map(|c| c.value).collect() })
    } else {
        None
    }
}

fn straight(cards: &[Card]) -> Option<Category> {
    let cards = cards.iter().sorted();
    let first = cards[0].value;
    if cards.iter().enumerate().skip(1).all(|(i, c)| {
        first + i as u8 == c.value
    })
    {
        Some(Straight { highest: cards[4].value })
    } else if cards.iter().map(|c| c.value).collect_vec() == &[2, 3, 4, 5, 14] {
        Some(Straight { highest: 5 })
    } else {
        None
    }
}

fn three_of_a_kind(cards: &[Card]) -> Option<Category> {
    let cards = cards.iter().sorted();
    if cards[0].value == cards[2].value {
        Some(ThreeOfAKind {
            triplet: cards[0].value,
            kickers: vec![cards[4].value, cards[3].value],
        })
    } else if cards[1].value == cards[3].value {
        Some(ThreeOfAKind {
            triplet: cards[1].value,
            kickers: vec![cards[4].value, cards[0].value],
        })
    } else if cards[2].value == cards[4].value {
        Some(ThreeOfAKind {
            triplet: cards[2].value,
            kickers: vec![cards[1].value, cards[0].value],
        })
    } else {
        None
    }
}

fn two_pair(cards: &[Card]) -> Option<Category> {
    let cards = cards.iter().sorted();
    let (pair1, pair2, kicker) = if cards[0].value == cards[1].value &&
        cards[2].value == cards[3].value
    {
        (cards[0].value, cards[2].value, cards[4].value)
    } else if cards[0].value == cards[1].value &&
               cards[3].value == cards[4].value
    {
        (cards[0].value, cards[3].value, cards[2].value)
    } else if cards[1].value == cards[2].value &&
               cards[3].value == cards[4].value
    {
        (cards[1].value, cards[3].value, cards[0].value)
    } else {
        return None;
    };

    Some(TwoPair {
        high: pair1.max(pair2),
        low: pair1.min(pair2),
        kicker: kicker,
    })
}

fn one_pair(cards: &[Card]) -> Option<Category> {
    let cards = cards.iter().sorted();
    let pair_value = if cards[0].value == cards[1].value {
        cards[0].value
    } else if cards[1].value == cards[2].value {
        cards[1].value
    } else if cards[2].value == cards[3].value {
        cards[2].value
    } else if cards[3].value == cards[4].value {
        cards[3].value
    } else {
        return None;
    };

    Some(OnePair {
        pair: pair_value,
        kickers: cards
            .iter()
            .map(|c| c.value)
            .filter(|&v| v != pair_value)
            .sorted_by(|c1, c2| c2.cmp(&c1)),
    })
}

fn high_card(cards: &[Card]) -> Category {
    let kickers: Vec<_> = cards.iter().map(|c| c.value).sorted_by(
        |c1, c2| c2.cmp(&c1),
    );
    HighCard { kickers: kickers }
}
