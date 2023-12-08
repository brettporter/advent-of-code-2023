use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use num::Integer;

advent_of_code::solution!(8);

fn parse_maps(input: &str) -> (Vec<usize>, HashMap<String, [String; 2]>) {
    let p = parser!(
        line(char_of("LR")+)
        line("")
        lines(string(alnum+) " = (" string(alnum+) ", " string(alnum+) ")")
    );

    let (instructions, _, maps_definition) = p.parse(input).unwrap();

    let mut maps = HashMap::new();
    for map in maps_definition {
        maps.insert(map.0, [map.1, map.2]);
    }
    (instructions, maps)
}

fn traverse_map(
    start: &str,
    instructions: &Vec<usize>,
    maps: &HashMap<String, [String; 2]>,
) -> usize {
    let mut pos = start;
    let mut count = 0;
    while !pos.ends_with('Z') {
        let inst = instructions[count % instructions.len()];
        pos = &maps[pos][inst];
        count += 1;
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, maps) = parse_maps(input);

    let count = traverse_map("AAA", &instructions, &maps);

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, maps) = parse_maps(input);

    let starts = maps.keys().filter(|k| k.ends_with('A')).collect_vec();
    let counts = starts
        .iter()
        .map(|start| traverse_map(start, &instructions, &maps));

    counts.reduce(|acc, e| acc.lcm(&e)).map(|x| x as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(2));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 12,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 21,
        ));
        assert_eq!(result, Some(6));
    }
}
