use std::iter;

use crate::helpers::ascii_to_digit;

/// Simple wrapper of [char::is_ascii_digit] to not require a reference
fn is_ascii_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn digit_at(s: &str, i: usize) -> u32 {
    ascii_to_digit(s.as_bytes()[i])
}

#[derive(Eq)]
struct FoundDigit {
    digit: u32,
    index: usize,
}

impl Ord for FoundDigit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for FoundDigit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FoundDigit {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl FoundDigit {
    fn first(input: &str) -> Option<FoundDigit> {
        let index = input.find(is_ascii_digit)?;
        Some(FoundDigit {
            digit: digit_at(input, index),
            index,
        })
    }

    fn last(input: &str) -> Option<FoundDigit> {
        let index = input.rfind(is_ascii_digit)?;
        Some(FoundDigit {
            digit: digit_at(input, index),
            index,
        })
    }
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = FoundDigit::first(line).unwrap();
            let last = FoundDigit::last(line).unwrap();
            (first.digit * 10) + last.digit
        })
        .sum()
}

struct NamedDigit {
    digit: u32,
    name: &'static str,
}

impl NamedDigit {
    const fn new(digit: u32, name: &'static str) -> Self {
        NamedDigit { digit, name }
    }
}

impl NamedDigit {
    fn find_in(&self, input: &str) -> Option<FoundDigit> {
        let index = input.find(self.name)?;
        Some(FoundDigit {
            digit: self.digit,
            index,
        })
    }

    fn rfind_in(&self, input: &str) -> Option<FoundDigit> {
        let index = input.rfind(self.name)?;
        Some(FoundDigit {
            digit: self.digit,
            index,
        })
    }
}

static NUM_NAMES: [NamedDigit; 9] = [
    NamedDigit::new(1, "one"),
    NamedDigit::new(2, "two"),
    NamedDigit::new(3, "three"),
    NamedDigit::new(4, "four"),
    NamedDigit::new(5, "five"),
    NamedDigit::new(6, "six"),
    NamedDigit::new(7, "seven"),
    NamedDigit::new(8, "eight"),
    NamedDigit::new(9, "nine"),
];

#[aoc(day1, part2, search_all)]
pub fn solve_part2_search_all(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = NUM_NAMES
                .iter()
                .map(|digit| digit.find_in(line))
                .chain(iter::once(FoundDigit::first(line)))
                .flatten()
                .min()
                .unwrap();

            let last = NUM_NAMES
                .iter()
                .map(|digit| digit.rfind_in(line))
                .chain(iter::once(FoundDigit::last(line)))
                .flatten()
                .max()
                .unwrap();

            (first.digit * 10) + last.digit
        })
        .sum()
}

// TODO: Find a faster way to implement part 2
