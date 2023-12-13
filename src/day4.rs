use crate::helpers::parse_vec;

#[aoc_generator(day4)]
pub fn input_gen(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winners, plays) = numbers.split_once(" | ").unwrap();
            let winners = parse_vec::<u32>(winners, " ");

            plays
                .split(" ")
                .flat_map(str::parse::<u32>)
                .filter(|play| winners.contains(play))
                .count()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(wins: &[usize]) -> u32 {
    wins.iter()
        .copied()
        .map(|wins| if wins == 0 { 0 } else { 1 << (wins - 1) })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(wins: &[usize]) -> u32 {
    let mut card_counts = vec![1u32; wins.len()];
    wins.iter()
        .copied()
        .enumerate()
        .map(|(index, wins)| {
            let dups = card_counts[index];

            for offset in 1..=wins {
                card_counts[index + offset] += dups;
            }

            dups
        })
        .sum()
}
