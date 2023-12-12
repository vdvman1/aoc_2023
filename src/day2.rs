#[derive(Default, Debug)]
pub struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (id, sets) = line.split_once(": ").unwrap();
            let id: u32 = id["Game ".len()..].parse().unwrap();

            let sets = sets
                .split("; ")
                .map(|set| {
                    let mut cubes = CubeSet::default();

                    for color in set.split(", ") {
                        let (count, color) = color.split_once(' ').unwrap();
                        let count: u32 = count.parse().unwrap();
                        match color {
                            "red" => cubes.red += count,
                            "green" => cubes.green += count,
                            "blue" => cubes.blue += count,
                            _ => panic!("Unexpected color: {}", color),
                        };
                    }

                    cubes
                })
                .collect();

            Game { id, sets }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    const MAX: CubeSet = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .iter()
        .filter(|game| {
            game.sets
                .iter()
                .all(|set| set.red <= MAX.red && set.green <= MAX.green && set.blue <= MAX.blue)
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            let set = game
                .sets
                .iter()
                .fold(CubeSet::default(), |total, set| CubeSet {
                    red: total.red.max(set.red),
                    green: total.green.max(set.green),
                    blue: total.blue.max(set.blue),
                });
            set.red * set.green * set.blue
        })
        .sum()
}
