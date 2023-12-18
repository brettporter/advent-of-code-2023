use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use num::FromPrimitive;

#[macro_use]
extern crate num_derive;

advent_of_code::solution!(10);

#[derive(FromPrimitive, Debug, PartialEq)]
enum Pipe {
    NS = 0,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

fn find_start(pipes: &Vec<Vec<Pipe>>) -> (usize, usize) {
    for (y, row) in pipes.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if *pipe == Pipe::Start {
                return (x, y);
            }
        }
    }
    panic!("Didn't find start");
}

fn find_pipe_from_start(pipes: &Vec<Vec<Pipe>>, start_x: usize, start_y: usize) -> Direction {
    // We're not told what type of pipe the start is, so look at the surrounding pipes
    // to determine what the valid connections are.

    let mut directions = Vec::new();

    // Check north
    if start_y > 0 {
        let pipe = &pipes[start_y - 1][start_x];
        if *pipe == Pipe::NS || *pipe == Pipe::SW || *pipe == Pipe::SE {
            directions.push(Direction::N);
        }
    }
    // Check south
    if start_y < pipes.len() - 1 {
        let pipe = &pipes[start_y + 1][start_x];
        if *pipe == Pipe::NS || *pipe == Pipe::NW || *pipe == Pipe::NE {
            directions.push(Direction::S);
        }
    }
    // Check east
    if start_x < pipes[0].len() - 1 {
        let pipe = &pipes[start_y][start_x + 1];
        if *pipe == Pipe::NW || *pipe == Pipe::SW || *pipe == Pipe::EW {
            directions.push(Direction::E);
        }
    }
    // Check west
    if start_x > 0 {
        let pipe = &pipes[start_y][start_x - 1];
        if *pipe == Pipe::NE || *pipe == Pipe::SE || *pipe == Pipe::EW {
            directions.push(Direction::W);
        }
    }

    // Return the pipe type, but also one of the valid directions to use as a starting direction
    directions.first().unwrap().clone()
}

fn pipe_direction(direction: &Direction, pipe: &Pipe) -> Direction {
    // Given we are entering the pipe in the given direction, determine
    // what the next direction will be based on the pipe type
    match pipe {
        Pipe::NS => {
            if *direction == Direction::N {
                Direction::N
            } else {
                Direction::S
            }
        }
        Pipe::EW => {
            if *direction == Direction::E {
                Direction::E
            } else {
                Direction::W
            }
        }
        Pipe::NE => {
            if *direction == Direction::S {
                Direction::E
            } else {
                Direction::N
            }
        }
        Pipe::NW => {
            if *direction == Direction::S {
                Direction::W
            } else {
                Direction::N
            }
        }
        Pipe::SW => {
            if *direction == Direction::N {
                Direction::W
            } else {
                Direction::S
            }
        }
        Pipe::SE => {
            if *direction == Direction::N {
                Direction::E
            } else {
                Direction::S
            }
        }
        _ => panic!("Should not hit unconnected pipe"),
    }
}

fn traverse_map(input: &str) -> (i32, usize) {
    let p = parser!(lines(
        line:char_of("|-LJ7F.S")+ => line.into_iter().map(|pipe| -> Pipe { FromPrimitive::from_usize(pipe).unwrap()} ).collect_vec()
    ));

    let pipes = p.parse(input).unwrap();

    // Find start
    let (start_x, start_y) = find_start(&pipes);

    // Find the type of pipe start is, and an available direction
    let mut direction = find_pipe_from_start(&pipes, start_x, start_y);

    // Navigate the pipe from the starting point until we return
    // to the starting point, calling process closure on each location
    let start_pos = (start_x as isize, start_y as isize);
    let mut pos = start_pos;
    let mut perimeter = 0;
    let mut area = 0;
    loop {
        perimeter += 1;
        let new_pos = match &direction {
            Direction::N => (pos.0, pos.1 - 1),
            Direction::S => (pos.0, pos.1 + 1),
            Direction::E => (pos.0 + 1, pos.1),
            Direction::W => (pos.0 - 1, pos.1),
        };
        if new_pos == start_pos {
            break;
        }
        area += (new_pos.0 + pos.0) * (new_pos.1 - pos.1);
        pos = new_pos;

        let pipe = &pipes[pos.1 as usize][pos.0 as usize];
        direction = pipe_direction(&direction, &pipe);
    }
    (perimeter, area.abs() as usize / 2)
}

pub fn part_one(input: &str) -> Option<u32> {
    // Traverse the main pipe and count the steps
    let (perimeter, _) = traverse_map(input);

    // Return half the steps, as this is the furthest point from the start
    Some(perimeter as u32 / 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    // Traverse the main pipe and calculate the area and perimeter
    let (perimeter, area) = traverse_map(input);
    // The area measures from the middle of each block in the perimeter, so we remove half the perimeter from the
    // calculated area to get the inside, then add 1 for the 4 outermost corners
    return Some(area - perimeter as usize / 2 + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 12,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 21,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 22,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 23,
        ));
        assert_eq!(result, Some(8));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 24,
        ));
        assert_eq!(result, Some(10));
    }
}
