use super::{parse_hand_bids, sum_scaled_bids, HandKind};

/// Represents a single card, with the variants ordered by the "strength" of the card
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Card {
    Joker = 0,
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T, // No clue what card this label represents
    Queen,
    King,
    Ace,
}

impl_card_from_u8!();

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Hand(HandKind, [Card; 5]);

impl super::Hand for Hand {
    type Card = Card;

    fn new(cards: [Card; 5]) -> Self {
        let mut counts = [0u8; Card::Ace as usize + 1];
        for card in cards {
            counts[card as usize] += 1;
        }

        let joker_count = counts[Card::Joker as usize];
        let kind = if joker_count == 5 {
            HandKind::FiveOfAKind
        } else {
            let mut dups = counts
                .iter()
                .skip(Card::Joker as usize + 1) // Don't include joker count, it's added to the first non-joker count
                .copied()
                .filter(|count| *count > 1);

            HandKind::from_dup_counts(
                dups.next().map_or(
                    joker_count + 1, /* include skipped ones */
                    |count| count + joker_count,
                ),
                dups.next(),
            )
        };

        Hand(kind, cards)
    }
}

#[aoc_generator(day7, part2)]
pub fn input_gen(input: &str) -> Vec<(Hand, u32)> {
    parse_hand_bids(input)
}

#[aoc(day7, part2)]
pub fn solve_part2(hand_bids: &[(Hand, u32)]) -> u32 {
    sum_scaled_bids(hand_bids)
}
