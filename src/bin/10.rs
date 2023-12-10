use std::collections::VecDeque;

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

fn find_pipe_from_start(
    pipes: &Vec<Vec<Pipe>>,
    start_x: usize,
    start_y: usize,
) -> (Pipe, Direction) {
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

    let pipe = match directions[0] {
        Direction::N => match directions[1] {
            Direction::S => Pipe::NS,
            Direction::E => Pipe::NE,
            Direction::W => Pipe::NW,
            _ => panic!("Invalid pipe"),
        },
        Direction::S => match directions[1] {
            Direction::N => Pipe::NS,
            Direction::E => Pipe::SE,
            Direction::W => Pipe::SW,
            _ => panic!("Invalid pipe"),
        },
        Direction::E => match directions[1] {
            Direction::N => Pipe::NE,
            Direction::S => Pipe::SE,
            Direction::W => Pipe::EW,
            _ => panic!("Invalid pipe"),
        },
        Direction::W => match directions[1] {
            Direction::N => Pipe::NW,
            Direction::S => Pipe::SW,
            Direction::E => Pipe::EW,
            _ => panic!("Invalid pipe"),
        },
    };

    (pipe, directions.first().unwrap().clone())
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

fn traverse_map<F>(input: &str, process: &mut F)
where
    F: FnMut(usize, usize, &Pipe),
{
    let p = parser!(lines(
        line:char_of("|-LJ7F.S")+ => line.into_iter().map(|pipe| -> Pipe { FromPrimitive::from_usize(pipe).unwrap()} ).collect_vec()
    ));

    let pipes = p.parse(input).unwrap();

    // Find start
    let (start_x, start_y) = find_start(&pipes);

    // Find an available direction
    let (start_pipe, mut direction) = find_pipe_from_start(&pipes, start_x, start_y);
    let mut pipe = &start_pipe;

    // Follow direction until return and then return half
    let (mut x, mut y) = (start_x, start_y);
    loop {
        process(x, y, &pipe);
        match &direction {
            Direction::N => y -= 1,
            Direction::S => y += 1,
            Direction::E => x += 1,
            Direction::W => x -= 1,
        }
        if x == start_x && y == start_y {
            break;
        }
        pipe = &pipes[y][x];
        direction = pipe_direction(&direction, &pipe);
    }
}

fn draw_pipe(seen: &mut Vec<Vec<bool>>, x: usize, y: usize, pipe: &Pipe) {
    let pattern = match pipe {
        Pipe::NS => [0, 1, 0, 0, 1, 0, 0, 1, 0],
        Pipe::EW => [0, 0, 0, 1, 1, 1, 0, 0, 0],
        Pipe::NE => [0, 1, 0, 0, 1, 1, 0, 0, 0],
        Pipe::NW => [0, 1, 0, 1, 1, 0, 0, 0, 0],
        Pipe::SW => [0, 0, 0, 1, 1, 0, 0, 1, 0],
        Pipe::SE => [0, 0, 0, 0, 1, 1, 0, 1, 0],
        _ => panic!("Can't draw this pipe {:?}", pipe),
    };
    for y_off in 0..3 {
        for x_off in 0..3 {
            if pattern[y_off * 3 + x_off] == 1 {
                seen[y * 3 + y_off][x * 3 + x_off] = true;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;

    traverse_map(input, &mut |_, _, _| count += 1);

    Some(count / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Assumption that input is < 150 wide and high
    const SIZE: usize = 150 * 3;
    let mut seen = vec![vec![false; SIZE]; SIZE];

    traverse_map(input, &mut |x, y, pipe| draw_pipe(&mut seen, x, y, pipe));

    let mut to_visit = VecDeque::new();
    // Assumption that this is outside... may need to adjust if not
    to_visit.push_back((0, 0));

    while let Some((x, y)) = to_visit.pop_back() {
        if seen[y][x] {
            continue;
        }

        seen[y][x] = true;
        if x > 0 {
            to_visit.push_back((x - 1, y));
        }
        if x < SIZE - 1 {
            to_visit.push_back((x + 1, y));
        }
        if y > 0 {
            to_visit.push_back((x, y - 1));
        }
        if y < SIZE - 1 {
            to_visit.push_back((x, y + 1));
        }
    }

    let mut count = 0;
    for y in 0..SIZE / 3 {
        for x in 0..SIZE / 3 {
            // TODO: maybe just check if that map square is ground?
            let mut ground = true;
            for check_y in 0..3 {
                for check_x in 0..3 {
                    if seen[y * 3 + check_y][x * 3 + check_x] {
                        ground = false;
                    }
                }
            }
            if ground {
                count += 1;
            }
        }
    }

    Some(count)
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
