#[derive(Default)]
pub struct Race {
    time: u64,
    distance: u64,
}

#[aoc_generator(day6, part1)]
pub fn input_gen1(input: &str) -> Vec<Race> {
    let mut rows = input.lines().map(|line| {
        line.split_ascii_whitespace() // all the fields are separated by one or more spaces
            .skip(1) // Skip row name, including the colon
            .flat_map(str::parse::<u64>)
    });

    // The order of calls to next is important, do not rearrange
    let times = rows.next().unwrap();
    let distances = rows.next().unwrap();

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn count_ways_to_win(race: &Race) -> u64 {
    // d(h, t) = distance mm travelled in race duration "t" ms if button is held for duration "h" ms
    //         = (time left after holding) * (velocity from holding)
    //         = (t - h) * h
    //         = th - h^2
    //         = -h^2 + th

    // This is a downwards facing parabola
    // It will have two intersections with the goal distance "g" mm along the "h" axis
    // The values larger than "g" will be between the two intersections
    // "t" and "g" are considered constants as they are provided from the race
    // g = record distance + 1

    // solving for "h" in d(h, t) = g
    // -h^2 + th = g
    // -h^2 + th - g = 0

    // Using the quadratic formula
    // h = (-b ± sqrt(b^2 - 4ac)) / 2a
    //   = ( -t ± sqrt( t^2 - 4(-1)(-g) ) ) / 2(-1)
    //   = ( -t ± sqrt( t^2 - 4g ) ) / -2
    //   = ( t ∓ sqrt( t^2 - 4g ) ) / 2
    // i = sqrt(t^2 - 4g)
    // h₁ = ceil( (t - i) / 2 )
    // h₂ = floor( (t + i) / 2 )
    // Number of ways to win = h₂ - h₁ + 1

    let t = race.time as f64;
    let discriminator = t * t - 4.0 * ((race.distance + 1) as f64);
    let sqrt_discriminator = discriminator.sqrt();

    let hold_min = (t - sqrt_discriminator) / 2.0;
    let hold_min = hold_min.ceil() as u64;

    let hold_max = (t + sqrt_discriminator) / 2.0;
    let hold_max = hold_max.floor() as u64;

    hold_max - hold_min + 1
}

#[aoc(day6, part1)]
pub fn solve_part1(races: &[Race]) -> u64 {
    races.iter().map(count_ways_to_win).product()
}

#[aoc_generator(day6, part2)]
pub fn input_gen2(input: &str) -> Race {
    let mut rows = input.lines().map(|line| {
        line.split_ascii_whitespace() // all the fields are separated by one or more spaces
            .skip(1) // Skip row name, including the colon
            .collect::<String>() // Combine all digits into a single string
            .parse::<u64>()
            .unwrap()
    });

    let mut race = Race::default();
    // The order of calls to next is important, do not rearrange
    race.time = rows.next().unwrap();
    race.distance = rows.next().unwrap();

    race
}

#[aoc(day6, part2)]
pub fn solve_part2(race: &Race) -> u64 {
    count_ways_to_win(race)
}
