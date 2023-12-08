use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let p = parser!(
        line(char_of("LR")+)
        line("")
        lines(string(upper+) " = (" string(upper+) ", " string(upper+) ")")
    );

    let (instructions, _, maps_definition) = p.parse(input).unwrap();

    let mut maps = HashMap::new();
    for map in maps_definition {
        maps.insert(map.0, [map.1, map.2]);
    }

    let mut pos = "AAA";
    let mut count = 0;
    while pos != "ZZZ" {
        let inst = instructions[count % instructions.len()];
        pos = &maps[pos][inst];
        count += 1;
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
