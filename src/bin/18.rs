use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(18);

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

fn calculate_area(instructions: Vec<(usize, isize)>) -> usize {
    let mut current_vertex = (0, 0);

    // Calculate the area, including the 1x1 size of the perimeter
    // Area inside a polygon = sum of ((x2 + x1) * (y2 - y1))/2 for each pair of vertices
    // To include the width of the perimeter, we add perimeter/2 + 1 for the starting square

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

    // abs value taken as counter-clockwise produces a negative area
    (area.abs() + perimeter) as usize / 2 + 1
}

fn hex_to_pair(s: &str) -> (usize, isize) {
    // First 5 characters are hexadecimal distance, last character is direction
    (
        usize::from_str_radix(&s[5..], 10).unwrap(),
        isize::from_str_radix(&s[0..5], 16).unwrap(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let p = parser!(lines(
        char_of("RDLU") " " i32 " (#" string(alnum+) ")"
    ));

    // Get the instructions as pairs of the (direction, distance)
    let instructions = p
        .parse(input.trim())
        .unwrap()
        .iter()
        .map(|inst| (inst.0, inst.1 as isize))
        .collect();

    Some(calculate_area(instructions))
}

pub fn part_two(input: &str) -> Option<usize> {
    let p = parser!(lines(
        char_of("RDLU") " " i32 " (#" string(alnum+) ")"
    ));

    // Get the instructions as colours only and parse into (distance, direction)
    let instructions = p
        .parse(input.trim())
        .unwrap()
        .iter()
        .map(|inst| hex_to_pair(inst.2.as_str()))
        .collect();

    Some(calculate_area(instructions))
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
        assert_eq!(hex_to_pair("0dc571"), (DOWN, 56407));
        assert_eq!(hex_to_pair("5713f0"), (RIGHT, 356671));
        assert_eq!(hex_to_pair("d2c081"), (DOWN, 863240));
        assert_eq!(hex_to_pair("59c680"), (RIGHT, 367720));
        assert_eq!(hex_to_pair("411b91"), (DOWN, 266681));
        assert_eq!(hex_to_pair("8ceee2"), (LEFT, 577262));
        assert_eq!(hex_to_pair("caa173"), (UP, 829975));
        assert_eq!(hex_to_pair("1b58a2"), (LEFT, 112010));
        assert_eq!(hex_to_pair("caa171"), (DOWN, 829975));
        assert_eq!(hex_to_pair("7807d2"), (LEFT, 491645));
        assert_eq!(hex_to_pair("a77fa3"), (UP, 686074));
        assert_eq!(hex_to_pair("015232"), (LEFT, 5411));
        assert_eq!(hex_to_pair("7a21e3"), (UP, 500254));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
