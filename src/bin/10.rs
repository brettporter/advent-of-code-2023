use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use num::FromPrimitive;

extern crate num;
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

#[derive(Debug, PartialEq)]
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

fn find_direction_from_start(pipes: &Vec<Vec<Pipe>>, start_x: usize, start_y: usize) -> Direction {
    // Check north
    if start_y > 0 {
        let pipe = &pipes[start_y - 1][start_x];
        if *pipe == Pipe::NS || *pipe == Pipe::SW || *pipe == Pipe::SE {
            return Direction::N;
        }
    }
    // Check south
    if start_y < pipes.len() - 1 {
        let pipe = &pipes[start_y + 1][start_x];
        if *pipe == Pipe::NS || *pipe == Pipe::NW || *pipe == Pipe::NE {
            return Direction::S;
        }
    }
    // Check east
    if start_x < pipes[0].len() - 1 {
        let pipe = &pipes[start_y][start_x + 1];
        if *pipe == Pipe::NW || *pipe == Pipe::SW || *pipe == Pipe::EW {
            return Direction::E;
        }
    }
    // Check west
    if start_x > 0 {
        let pipe = &pipes[start_y][start_x - 1];
        if *pipe == Pipe::NE || *pipe == Pipe::SE || *pipe == Pipe::EW {
            return Direction::W;
        }
    }
    panic!("Didn't find a direction from the start");
}

fn pipe_direction(direction: &Direction, pipe: &Pipe) -> Direction {
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

pub fn part_one(input: &str) -> Option<u32> {
    let p = parser!(lines(
        line:char_of("|-LJ7F.S")+ => line.into_iter().map(|pipe| -> Pipe { FromPrimitive::from_usize(pipe).unwrap()} ).collect_vec()
    ));

    let pipes = p.parse(input).unwrap();

    // Find start
    let (start_x, start_y) = find_start(&pipes);

    // Find an available direction
    let mut direction = find_direction_from_start(&pipes, start_x, start_y);

    // Follow direction until return and then return half
    let (mut x, mut y) = (start_x, start_y);
    let mut count = 0;
    loop {
        count += 1;
        match &direction {
            Direction::N => y -= 1,
            Direction::S => y += 1,
            Direction::E => x += 1,
            Direction::W => x -= 1,
        }
        if x == start_x && y == start_y {
            break;
        }
        direction = pipe_direction(&direction, &pipes[y][x]);
    }

    Some(count / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
