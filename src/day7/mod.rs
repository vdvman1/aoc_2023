macro_rules! impl_card_from_u8 {
    () => {
        impl From<u8> for Card {
            fn from(value: u8) -> Self {
                match value {
                    b'2'..=b'9' => unsafe { std::mem::transmute(value - b'0') },
                    b'T' => Card::T,
                    b'J' => Card::Joker,
                    b'Q' => Card::Queen,
                    b'K' => Card::King,
                    b'A' => Card::Ace,
                    _ => panic!("Unknown card label: {}", value),
                }
            }
        }
    };
}

mod part1;
mod part2;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
pub enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    fn from_dup_counts(count1: u8, count2: Option<u8>) -> HandKind {
        match (count1, count2) {
            (5, _ /* None, 5 is max */) => HandKind::FiveOfAKind,
            (4, _ /* None, 1 is filtered */) => HandKind::FourOfAKind,
            (3, Some(2)) | (2, Some(3)) => HandKind::FullHouse,
            (3, _ /* None, 1,1 is filtered */) => HandKind::ThreeOfAKind,
            (2, Some(2)) => HandKind::TwoPair,
            (2, _ /* None, 1,1,1 is filtered */) => HandKind::OnePair,
            (1, _ /* None, 1,1,1,1 is filtered */) => HandKind::HighCard,
            _ => panic!("Unexpected dup counts"),
        }
    }
}

pub trait Hand: Ord + Clone {
    type Card: Ord + Copy + From<u8>;

    fn new(cards: [Self::Card; 5]) -> Self;
}

fn parse_hand_bids<H: Hand>(input: &str) -> Vec<(H, u32)> {
    input
        .lines()
        .map(|line| {
            let (labels, bid) = line.split_once(" ").unwrap();
            let bid: u32 = bid.parse().unwrap();
            let labels: [u8; 5] = labels.as_bytes().try_into().unwrap();
            (Hand::new(labels.map(H::Card::from)), bid)
        })
        .collect()
}

fn sum_scaled_bids<H: Ord + Clone>(hand_bids: &[(H, u32)]) -> u32 {
    let mut hand_bids = Vec::from(hand_bids);
    hand_bids.sort_unstable_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));
    hand_bids
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| *bid * (index as u32 + 1))
        .sum()
}
