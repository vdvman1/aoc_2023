use crate::helpers::{parse_vec, RangeMap};

// TODO: There are bound to be optimisations, part 2 takes 144 seconds to run!

#[derive(Default)]
pub struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: RangeMap<u32, u32>,
    soil_to_fertiliser: RangeMap<u32, u32>,
    fertiliser_to_water: RangeMap<u32, u32>,
    water_to_light: RangeMap<u32, u32>,
    light_to_temp: RangeMap<u32, u32>,
    temp_to_humidity: RangeMap<u32, u32>,
    humidity_to_location: RangeMap<u32, u32>,
}

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> Almanac {
    let mut lines = input.lines();

    let (_, seeds) = lines.next().unwrap().split_once(": ").unwrap();

    let mut almanac = Almanac::default();
    almanac.seeds = parse_vec(seeds, " ");

    lines.next(); // Skip first empty line

    fn gen_map<'a>(lines: &mut impl Iterator<Item = &'a str>) -> RangeMap<u32, u32> {
        lines.next(); // Skip map name line
        RangeMap::from_iter(
            lines
                .by_ref()
                .take_while(|line| line.len() > 0) // Take all lines until an empty line, and skip the empty line
                .map(|line| {
                    let (dest_start, rest) = line.split_once(" ").unwrap();
                    let (source_start, len) = rest.split_once(" ").unwrap();

                    let dest_start: u32 = dest_start.parse().unwrap();
                    let source_start: u32 = source_start.parse().unwrap();
                    let len: u32 = len.parse().unwrap();

                    (source_start..=(source_start + len - 1), dest_start)
                }),
        )
    }

    // The order of calls to gen_map is important, do not rearrange
    almanac.seed_to_soil = gen_map(&mut lines);
    almanac.soil_to_fertiliser = gen_map(&mut lines);
    almanac.fertiliser_to_water = gen_map(&mut lines);
    almanac.water_to_light = gen_map(&mut lines);
    almanac.light_to_temp = gen_map(&mut lines);
    almanac.temp_to_humidity = gen_map(&mut lines);
    almanac.humidity_to_location = gen_map(&mut lines);

    almanac
}

fn find_lowest_location(almanac: &Almanac, seeds: impl Iterator<Item = u32>) -> u32 {
    seeds
        .map(|seed| {
            let soil = almanac.seed_to_soil.get_and_offset_or_key(seed);
            let fertiliser = almanac.soil_to_fertiliser.get_and_offset_or_key(soil);
            let water = almanac
                .fertiliser_to_water
                .get_and_offset_or_key(fertiliser);
            let light = almanac.water_to_light.get_and_offset_or_key(water);
            let temp = almanac.light_to_temp.get_and_offset_or_key(light);
            let humidity = almanac.temp_to_humidity.get_and_offset_or_key(temp);
            almanac.humidity_to_location.get_and_offset_or_key(humidity)
        })
        .min()
        .unwrap()
}

#[aoc(day5, part1)]
pub fn solve_part1(almanac: &Almanac) -> u32 {
    find_lowest_location(almanac, almanac.seeds.iter().copied())
}

#[aoc(day5, part2)]
pub fn solve_part2(almanac: &Almanac) -> u32 {
    let seeds = almanac.seeds.chunks_exact(2).flat_map(|range| {
        let start = range[0];
        let len = range[1];

        start..(start + len)
    });

    find_lowest_location(almanac, seeds)
}
