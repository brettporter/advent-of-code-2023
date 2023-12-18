use aoc_parse::{parser, prelude::*};
use fxhash::FxHashMap;
use itertools::Itertools;
use num::Integer;

advent_of_code::solution!(8);

fn parse_maps(input: &str) -> (Vec<usize>, FxHashMap<String, [String; 2]>) {
    let p = parser!(
        line(char_of("LR")+)
        line("")
        lines(string(alnum+) " = (" string(alnum+) ", " string(alnum+) ")")
    );

    let (instructions, _, maps_definition) = p.parse(input).unwrap();

    let mut maps = FxHashMap::default();
    for map in maps_definition {
        maps.insert(map.0, [map.1, map.2]);
    }

    // instructions is a vector of 0 = L, 1 = R; maps is src => dest(L, R)
    (instructions, maps)
}

fn traverse_map(
    start: &str,
    instructions: &Vec<usize>,
    maps: &FxHashMap<String, [String; 2]>,
) -> usize {
    // calculate the distance from the start to a destination
    let mut pos = start;
    let mut count = 0;
    while !pos.ends_with('Z') {
        // get the next instruction, looping to the start after all are read
        let inst = instructions[count % instructions.len()];
        pos = &maps[pos][inst];
        count += 1;
    }
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let (instructions, maps) = parse_maps(input);

    let count = traverse_map("AAA", &instructions, &maps);

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (instructions, maps) = parse_maps(input);

    // calculate the distance to the destination of each possible start point
    let starts = maps.keys().filter(|k| k.ends_with('A')).collect_vec();
    let counts = starts
        .iter()
        .map(|start| traverse_map(start, &instructions, &maps));

    // Calculate the least common multiple of the routes - this will be the point in time that each
    // of the routes land at a destination at the same time.
    //
    // This works because of an assumption in the input data - it is structed such that continuing
    // to traverse past the destination will reach the same destination again in the same distance
    // as the original route.
    counts.reduce(|acc, e| acc.lcm(&e))
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
