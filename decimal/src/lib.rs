#![feature(conservative_impl_trait)]

extern crate itertools;
use itertools::Itertools;

use std::ops::{Add, Mul, Sub};
use std::cmp::Ordering;
use Sign::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Sign {
    Minus,
    Plus,
}

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, Eq)]
pub struct Decimal {
    sign: Sign,
    unsigned: UnsignedDecimal,
}

impl Decimal {
    pub fn try_from(s: &str) -> Option<Decimal> {
        let (sign, s) = match s.chars().nth(0) {
            Some('-') => (Minus, &s[1..]),
            Some('+') => (Plus, &s[1..]),
            Some('0'...'9') => (Plus, s),
            _ => return None,
        };

        let unsigned = match UnsignedDecimal::try_from(s) {
            Some(unsigned) => unsigned,
            None => return None,
        };

        Some(Decimal { sign, unsigned })
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Decimal) -> Ordering {
        match (self.sign, other.sign) {
            (Minus, Minus) => other.unsigned.cmp(&self.unsigned),
            (Minus, Plus) => Ordering::Less,
            (Plus, Minus) => Ordering::Greater,
            (Plus, Plus) => self.unsigned.cmp(&other.unsigned),
        }
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Decimal {
        let (sign, unsigned) = match (self.sign, other.sign) {
            (Minus, Minus) => (Minus, self.unsigned + other.unsigned),
            (Minus, Plus) => {
                match self.unsigned.cmp(&other.unsigned) {
                    Ordering::Less => (Plus, other.unsigned - self.unsigned),
                    _ => (Minus, self.unsigned - other.unsigned),
                }
            }
            (Plus, Minus) => {
                match self.unsigned.cmp(&other.unsigned) {
                    Ordering::Less => (Minus, other.unsigned - self.unsigned),
                    _ => (Plus, self.unsigned - other.unsigned),
                }
            }
            (Plus, Plus) => (Plus, self.unsigned + other.unsigned),
        };

        Decimal { sign, unsigned }
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Decimal {
        let (sign, unsigned) = match (self.sign, other.sign) {
            (Minus, Minus) => (Minus, self.unsigned + other.unsigned),
            (Minus, Plus) => (Minus, self.unsigned + other.unsigned),
            (Plus, Minus) => (Plus, self.unsigned + other.unsigned),
            (Plus, Plus) => {
                match self.unsigned.cmp(&other.unsigned) {
                    Ordering::Less => (Minus, other.unsigned - self.unsigned),
                    _ => (Plus, self.unsigned - other.unsigned),
                }
            }
        };

        Decimal { sign, unsigned }
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        let (sign, unsigned) = match (self.sign, other.sign) {
            (Minus, Minus) => (Plus, self.unsigned * other.unsigned),
            (Minus, Plus) => (Minus, self.unsigned * other.unsigned),
            (Plus, Minus) => (Minus, self.unsigned * other.unsigned),
            (Plus, Plus) => (Plus, self.unsigned * other.unsigned),
        };

        Decimal { sign, unsigned }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct UnsignedDecimal {
    // 123 is stored as [1, 2, 3]
    integer: Vec<u8>,

    // 0.123 is stored as [1, 2, 3]
    fraction: Vec<u8>,
}

impl UnsignedDecimal {
    pub fn try_from(s: &str) -> Option<UnsignedDecimal> {
        let parts = s.split(".").collect_vec();
        if parts.len() == 0 || parts.len() >= 3 {
            return None;
        }

        if parts[0].chars().any(|c| !c.is_digit(10)) {
            return None;
        }

        let integer = parts[0]
            .trim_left_matches("0")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let fraction = if parts.len() == 2 {
            if parts[1].chars().any(|c| !c.is_digit(10)) {
                return None;
            }

            parts[1]
                .trim_right_matches("0")
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        } else {
            Vec::new()
        };

        Some(UnsignedDecimal { integer, fraction })
    }
}

impl Add for UnsignedDecimal {
    type Output = UnsignedDecimal;

    fn add(self, other: UnsignedDecimal) -> UnsignedDecimal {
        let nfractions = self.fraction.len().max(other.fraction.len());

        let (l, r): (Vec<_>, Vec<_>) = zip_pad_zero(
            self.fraction.iter().cloned(),
            other.fraction.iter().cloned(),
        ).unzip();

        let value = add(
            l.into_iter().rev().chain(self.integer.into_iter().rev()),
            r.into_iter().rev().chain(other.integer.into_iter().rev()),
        );

        let (integer, fraction) = split_decimal(&value, nfractions);

        UnsignedDecimal { integer, fraction }
    }
}

impl Sub for UnsignedDecimal {
    type Output = UnsignedDecimal;

    fn sub(self, other: UnsignedDecimal) -> UnsignedDecimal {
        debug_assert!(self > other);

        let nfractions = self.fraction.len().max(other.fraction.len());

        let (l, r): (Vec<_>, Vec<_>) = zip_pad_zero(
            self.fraction.iter().cloned(),
            other.fraction.iter().cloned(),
        ).unzip();

        let value = sub(
            l.into_iter().rev().chain(self.integer.into_iter().rev()),
            r.into_iter().rev().chain(other.integer.into_iter().rev()),
        );

        let (integer, fraction) = split_decimal(&value, nfractions);

        UnsignedDecimal { integer, fraction }
    }
}

impl Mul for UnsignedDecimal {
    type Output = UnsignedDecimal;

    fn mul(self, other: UnsignedDecimal) -> UnsignedDecimal {
        let nfractions = self.fraction.len() + other.fraction.len();
        let value = mul(
            self.fraction.into_iter().rev().chain(
                self.integer
                    .into_iter()
                    .rev(),
            ),
            other.fraction.into_iter().rev().chain(
                other
                    .integer
                    .into_iter()
                    .rev(),
            ),
        );

        let (integer, fraction) = split_decimal(&value, nfractions);

        UnsignedDecimal { integer, fraction }
    }
}

impl PartialOrd for UnsignedDecimal {
    fn partial_cmp(&self, other: &UnsignedDecimal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UnsignedDecimal {
    fn cmp(&self, other: &UnsignedDecimal) -> Ordering {
        self.integer
            .len()
            .cmp(&other.integer.len())
            .then(self.integer.iter().cmp(other.integer.iter()))
            .then(self.fraction.cmp(&other.fraction))
    }
}

fn zip_pad_zero<L, R>(left: L, right: R) -> impl Iterator<Item = (u8, u8)>
where
    L: Iterator<Item = u8>,
    R: Iterator<Item = u8>,
{
    use itertools::EitherOrBoth::*;
    left.zip_longest(right).map(|z| match z {
        Both(l, r) => (l, r),
        Right(r) => (0, r),
        Left(l) => (l, 0),
    })
}

// From LSB to MSB
fn add<L, R>(left: L, right: R) -> Vec<u8>
where
    L: Iterator<Item = u8>,
    R: Iterator<Item = u8>,
{
    let mut carry = 0;
    let mut vec = zip_pad_zero(left, right)
        .map(|(l, r)| {
            let val = l + r + carry;
            carry = val / 10;
            val % 10

        })
        .collect_vec();
    if carry != 0 {
        vec.push(carry);
    }
    vec
}

// From LSB to MSB
fn sub<L, R>(left: L, right: R) -> Vec<u8>
where
    L: Iterator<Item = u8>,
    R: Iterator<Item = u8>,
{
    let mut borrow = 0;
    let vec = zip_pad_zero(left, right)
        .map(|(l, r)| if l < r + borrow {
            let val = 10 + l - r - borrow;
            borrow = 1;
            val
        } else {
            let val = l - r - borrow;
            borrow = 0;
            val
        })
        .collect();
    debug_assert_eq!(borrow, 0);
    vec
}

// From LSB to MSB
fn mul<L, R>(left: L, right: R) -> Vec<u8>
where
    L: Iterator<Item = u8> + Clone,
    R: Iterator<Item = u8> + Clone,
{
    let mut carry = 0;
    let mut result = Vec::new();
    for (ri, r) in right.enumerate() {
        let mut val = left.clone()
            .map(|l| {
                let val = r * l + carry;
                carry = val / 10;
                val % 10
            })
            .collect_vec();
        if carry != 0 {
            val.push(carry);
        }
        let mut tmp = vec![0; ri];
        tmp.extend(val);
        val = tmp;

        result = add(result.iter().cloned(), val.iter().cloned());
    }

    result
}

// The first `nfractions` elements are the fractional part, and the
// remaining elements are the integral part.
fn split_decimal(value: &[u8], nfractions: usize) -> (Vec<u8>, Vec<u8>) {
    let (fraction, integer) = value.split_at(nfractions);
    let integer = integer
        .into_iter()
        .rev()
        .skip_while(|&&n| n == 0)
        .cloned()
        .collect();

    let mut fraction = fraction
        .into_iter()
        .skip_while(|&&n| n == 0)
        .cloned()
        .collect_vec();
    fraction.reverse();

    (integer, fraction)
}
