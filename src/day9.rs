use crate::helpers::parse_vec;

#[aoc_generator(day9)]
pub fn input_gen(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| parse_vec::<i32>(line, " "))
        .collect()
}

fn get_diffs<'a>(changes: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    changes.windows(2).map(|values| values[1] - values[0])
}

fn all_eq(diffs: &Vec<i32>) -> bool {
    diffs.windows(2).all(|values| values[0] == values[1])
}

fn each_diffs(changes: &Vec<i32>, mut callback: impl FnMut(&Vec<i32>) -> ()) {
    let mut diffs: &mut Vec<i32> = &mut get_diffs(changes).collect();
    callback(diffs);

    if !all_eq(diffs) {
        let mut next_diffs: &mut Vec<i32> = &mut get_diffs(diffs).collect();
        callback(next_diffs);

        while !all_eq(next_diffs) {
            std::mem::swap(&mut diffs, &mut next_diffs);
            next_diffs.clear();
            next_diffs.extend(get_diffs(diffs));
            callback(next_diffs);
        }
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(values_changes: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for changes in values_changes {
        sum += *changes.last().unwrap();
        each_diffs(changes, |diffs| sum += diffs.last().unwrap());
    }
    sum
}

#[aoc(day9, part2)]
pub fn solve_part2(values_changes: &Vec<Vec<i32>>) -> i32 {
    let mut start_values: Vec<i32> = Vec::new();

    values_changes
        .iter()
        .map(|changes| {
            start_values.push(changes[0]);
            each_diffs(changes, |diffs| start_values.push(diffs[0]));
            start_values
                .drain(..)
                .rev()
                .reduce(|diff, start_value| start_value - diff)
                .unwrap()
        })
        .sum()
}
