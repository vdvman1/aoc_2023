use crate::helpers::parse_vec;

pub struct Card {
    winners: Vec<u32>,
    plays: Vec<u32>,
}

#[aoc_generator(day4)]
pub fn input_gen(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winners, plays) = numbers.split_once(" | ").unwrap();
            Card {
                winners: parse_vec(winners, " "),
                plays: parse_vec(plays, " "),
            }
        })
        .collect()
}

fn count_wins(card: &Card) -> usize {
    card.plays
        .iter()
        .filter(|play| card.winners.contains(play))
        .count()
}

#[aoc(day4, part1)]
pub fn solve_part1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|card| {
            let count = count_wins(card);
            if count == 0 {
                0
            } else {
                1 << (count - 1)
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(cards: &[Card]) -> u32 {
    let mut card_counts = vec![1u32; cards.len()];
    cards
        .iter()
        .enumerate()
        .map(|(index, card)| {
            let count = count_wins(card);
            let dups = card_counts[index];

            for offset in 1..=count {
                card_counts[index + offset] += dups;
            }

            dups
        })
        .sum()
}
