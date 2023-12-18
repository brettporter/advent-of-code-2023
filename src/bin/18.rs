use std::collections::VecDeque;

use aoc_parse::{parser, prelude::*};
use itertools::Itertools;

advent_of_code::solution!(18);

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

pub fn part_one(input: &str) -> Option<u32> {
    let p = parser!(lines(
        char_of("UDLR") " " i32 " (#" string(alnum+) ")"
    ));

    let instructions = p.parse(input.trim()).unwrap();
    println!("{:?}", instructions);

    const SIZE: usize = 1000;
    let mut grid = vec![vec![0; SIZE]; SIZE];
    let mut pos = (SIZE / 2, SIZE / 2);
    grid[pos.1][pos.0] = 1;

    let (mut min_x, mut min_y) = (SIZE, SIZE);
    let (mut max_x, mut max_y) = (0, 0);
    for (dir, count, colour) in instructions {
        for i in 0..count {
            match dir {
                UP => pos.1 -= 1,
                DOWN => pos.1 += 1,
                LEFT => pos.0 -= 1,
                RIGHT => pos.0 += 1,
                _ => panic!("Invalid direction: {}", dir),
            }
            grid[pos.1][pos.0] = 1;

            min_x = min_x.min(pos.0);
            min_y = min_y.min(pos.1);
            max_x = max_x.max(pos.0);
            max_y = max_y.max(pos.1);
        }
    }

    flood_fill(&mut grid, (0, 0), 2);

    // for row in &grid {
    //     if let Some(min) = row.iter().position(|c| *c == 1) {
    //         let max = SIZE - 1 - row.iter().rev().position(|c| *c == 1).unwrap();
    //         println!("{} -> {}", min, max);
    //     }
    // }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                match grid[y][x] {
                    1 => '#',
                    0 => '.',
                    2 => ' ',
                    _ => panic!(),
                }
            );
        }
        println!();
    }
    println!();

    Some(
        grid.iter()
            .map(|row| row.iter().filter(|c| **c != 2).count())
            .sum::<usize>() as u32,
    )
}

fn flood_fill(grid: &mut Vec<Vec<i32>>, start: (usize, usize), value: i32) {
    let mut to_fill = VecDeque::new();
    to_fill.push_back(start);

    while let Some(c) = to_fill.pop_front() {
        if grid[c.1][c.0] != 0 {
            continue;
        }

        grid[c.1][c.0] = value;

        if c.0 > 0 {
            to_fill.push_back((c.0 - 1, c.1));
        }
        if c.0 < grid[0].len() - 1 {
            to_fill.push_back((c.0 + 1, c.1));
        }
        if c.1 > 0 {
            to_fill.push_back((c.0, c.1 - 1));
        }
        if c.1 < grid.len() - 1 {
            to_fill.push_back((c.0, c.1 + 1));
        }
    }
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
