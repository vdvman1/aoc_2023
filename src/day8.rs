use std::collections::HashMap;

use crate::helpers::{get_bytes_len, lcm};

type NodeName = [u8; 3];

#[derive(Default)]
pub struct Map {
    /// true means left, false means right
    steps: Vec<bool>,
    nodes: HashMap<NodeName, (NodeName, NodeName)>,
}

impl Map {
    fn steps_between(&self, start_id: [u8; 3], stop: impl Fn([u8; 3]) -> bool) -> usize {
        let mut node = self.nodes[&start_id];
        self.steps
            .iter()
            .copied()
            .cycle()
            .take_while(|take_left| {
                let id = if *take_left { node.0 } else { node.1 };
                if stop(id) {
                    false
                } else {
                    node = self.nodes[&id];
                    true
                }
            })
            .count()
            + 1
    }
}

#[aoc_generator(day8)]
pub fn input_gen(input: &str) -> Map {
    let mut lines = input.lines();
    let mut map = Map::default();

    map.steps = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .copied()
        .map(|c| c == b'L')
        .collect();

    lines.next(); // Ignore empty line

    map.nodes = HashMap::from_iter(lines.map(|line| {
        let id = get_bytes_len::<0, 3>(line);
        let left = get_bytes_len::<{ b"AAA = (".len() }, 3>(line);
        let right = get_bytes_len::<{ b"AAA = (AAA, ".len() }, 3>(line);

        (id, (left, right))
    }));

    map
}

#[aoc(day8, part1)]
pub fn solve_part1(map: &Map) -> usize {
    map.steps_between(*b"AAA", |id| id == *b"ZZZ")
}

#[aoc(day8, part2)]
pub fn solve_part2(map: &Map) -> usize {
    // Input data always forms loops that only reach a single end node, so we only need to calculate the length of each cycle, then find the least common multiple
    map.nodes
        .iter()
        .filter_map(|(id, _)| if id[2] == b'A' { Some(*id) } else { None })
        .map(|start_id| map.steps_between(start_id, |id| id[2] == b'Z'))
        .reduce(lcm)
        .unwrap()
}
