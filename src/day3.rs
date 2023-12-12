use std::{iter, mem};

use crate::helpers::{try_ascii_to_digit, Vec2d};

#[derive(Debug)]
pub enum Value {
    Part(usize),
    Gap,
    Symbol(u8),
}

#[derive(Default)]
pub struct Schematic {
    parts: Vec<u32>,
    grid: Vec2d<Value>,
}

#[aoc_generator(day3)]
pub fn input_gen(input: &str) -> Schematic {
    let mut schematic = Schematic::default();
    for line in input.lines() {
        let mut partial_part: Option<u32> = None;

        schematic
            .grid
            .add_row(line.bytes().chain(iter::once(b'\n')).filter_map(|c| {
                if let Some(digit) = try_ascii_to_digit(c) {
                    partial_part = Some(match partial_part {
                        None => digit,
                        Some(rest) => rest * 10 + digit,
                    });

                    Some(Value::Part(schematic.parts.len()))
                } else {
                    if let Some(part) = partial_part.take() {
                        schematic.parts.push(part);
                    }

                    match c {
                        b'.' => Some(Value::Gap),
                        b'\n' => None,
                        _ => Some(Value::Symbol(c)),
                    }
                }
            }))
    }

    schematic
}

static VALUES_AROUND: [fn(&Vec2d<Value>, usize, usize) -> Option<&Value>; 8] = [
    Vec2d::above_left,
    Vec2d::above,
    Vec2d::above_right,
    Vec2d::left,
    Vec2d::right,
    Vec2d::below_left,
    Vec2d::below,
    Vec2d::below_right,
];

#[aoc(day3, part1)]
pub fn solve_part1(schematic: &Schematic) -> u32 {
    let mut seen: Vec<bool> = vec![false; schematic.parts.len()];
    schematic
        .grid
        .iter()
        .filter(|(_, _, value)| matches!(value, Value::Symbol(_)))
        .map(|(x, y, _)| {
            VALUES_AROUND
                .iter()
                .map(|get| {
                    if let Some(Value::Part(index)) = get(&schematic.grid, x, y) {
                        if !mem::replace(&mut seen[*index], true) {
                            return schematic.parts[*index];
                        }
                    }

                    0
                })
                .sum::<u32>()
        })
        .sum()
}
