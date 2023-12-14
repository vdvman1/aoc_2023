pub struct Race {
    time: u32,
    distance: u32,
}

#[aoc_generator(day6, part1)]
pub fn input_gen1(input: &str) -> Vec<Race> {
    let mut rows = input.lines().map(|line| {
        line.split_ascii_whitespace() // all the fields are separated by one or more spaces
            .skip(1) // Skip row name, including the colon
            .flat_map(str::parse::<u32>)
    });
    let times = rows.next().unwrap();
    let distances = rows.next().unwrap();

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(races: &[Race]) -> u32 {
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

    races
        .iter()
        .map(|race| {
            let t = race.time as f64;
            let discriminator = t * t - 4.0 * ((race.distance + 1) as f64);
            let sqrt_discriminator = discriminator.sqrt();

            let hold_min = (t - sqrt_discriminator) / 2.0;
            let hold_min = hold_min.ceil() as u32;

            let hold_max = (t + sqrt_discriminator) / 2.0;
            let hold_max = hold_max.floor() as u32;

            hold_max - hold_min + 1
        })
        .product()
}
