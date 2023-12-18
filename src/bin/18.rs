use std::collections::VecDeque;

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(18);

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

pub fn part_one(input: &str) -> Option<usize> {
    let p = parser!(lines(
        char_of("RDLU") " " i32 " (#" string(alnum+) ")"
    ));

    let instructions = p
        .parse(input.trim())
        .unwrap()
        .iter()
        .map(|inst| (inst.0, inst.1 as isize))
        .collect();

    calculate_area(instructions, 1000)
}

fn calculate_area(instructions: Vec<(usize, isize)>, max_size: usize) -> Option<usize> {
    let mut current_vertex = (0, 0);

    let mut area = 0;
    let mut perimeter = 0;
    for (dir, count) in instructions {
        perimeter += count;
        let vertex = match dir {
            UP => (current_vertex.0, current_vertex.1 - count),
            DOWN => (current_vertex.0, current_vertex.1 + count),
            LEFT => (current_vertex.0 - count, current_vertex.1),
            RIGHT => (current_vertex.0 + count, current_vertex.1),
            _ => panic!("Invalid direction: {}", dir),
        };
        area += (current_vertex.0 + vertex.0) * (current_vertex.1 - vertex.1);
        current_vertex = vertex;
    }

    Some((area.abs() + perimeter) as usize / 2 + 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let p = parser!(lines(
        char_of("RDLU") " " i32 " (#" string(alnum+) ")"
    ));

    let instructions = p
        .parse(input.trim())
        .unwrap()
        .iter()
        .map(|inst| hex_to_pair(inst.2.as_str()))
        .collect();

    calculate_area(instructions, 1000000)
}

fn hex_to_pair(s: &str) -> (usize, isize) {
    (
        usize::from_str_radix(&s[5..], 10).unwrap(),
        isize::from_str_radix(&s[0..5], 16).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_hex_to_pair() {
        assert_eq!(hex_to_pair("70c710"), (RIGHT, 461937));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
