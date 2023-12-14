use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use seahash::{hash, SeaHasher};

advent_of_code::solution!(14);

const EMPTY: u8 = 0;
const SQUARE: u8 = 1;
const ROUND: u8 = 2;

pub fn part_one(input: &str) -> Option<u32> {
    let p = parser!(lines(
        char_of(".#O")+
    ));

    let mut grid = p.parse(input).unwrap();
    // let direction = (0, -1);

    let w = grid[0].len();
    let h = grid.len();
    let mut last_free = vec![0; w];

    // TODO: this is direction dependent...
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == SQUARE as usize {
                last_free[x] = y + 1;
            }
            if grid[y][x] == ROUND as usize {
                if last_free[x] < y {
                    // move up
                    grid[last_free[x]][x] = ROUND as usize;
                    grid[y][x] = EMPTY as usize;
                    last_free[x] += 1;
                } else {
                    last_free[x] = y + 1;
                }
            }
        }
    }

    let count = grid
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().filter(|r| **r == ROUND as usize).count() * (h - y))
        .sum::<usize>();
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let p = parser!(lines(
        char_of(".#O")+
    ));

    let map = p.parse(input).unwrap();
    // let direction = (0, -1);

    let w = map[0].len();
    let h = map.len();

    let mut grid = map.concat().into_iter().map(|b| b as u8).collect_vec();
    let mut seen = HashMap::new();

    print_grid(&grid, w, h);

    let mut target = 1000000000;
    let mut num = 0;
    while num < target {
        let k = hash(grid.as_slice());
        if let Some(prev) = seen.insert(k, num) {
            let interval = num - prev;
            let end = (1000000000 - prev) % interval;
            println!("Cycle {} {} -> {} {}", prev, num, end, prev + end);
            target = num + end;
        }

        let mut last_free = vec![0; w];

        // TODO: this is direction dependent...
        for y in 0..h {
            for x in 0..w {
                if grid[y * w + x] == SQUARE {
                    last_free[x] = y + 1;
                }
                if grid[y * w + x] == ROUND {
                    if last_free[x] < y {
                        // move up
                        grid[last_free[x] * w + x] = ROUND;
                        grid[y * w + x] = EMPTY;
                        last_free[x] += 1;
                    } else {
                        last_free[x] = y + 1;
                    }
                }
            }
        }

        let mut last_free = vec![0; h];

        // TODO: this is direction dependent...
        for x in 0..w {
            for y in 0..h {
                if grid[y * w + x] == SQUARE {
                    last_free[y] = x + 1;
                }
                if grid[y * w + x] == ROUND {
                    if last_free[y] < x {
                        // move west
                        grid[y * w + last_free[y]] = ROUND;
                        grid[y * w + x] = EMPTY;
                        last_free[y] += 1;
                    } else {
                        last_free[y] = x + 1;
                    }
                }
            }
        }
        let mut last_free = vec![h - 1; w];

        // TODO: this is direction dependent...
        for y in (0..h).rev() {
            for x in 0..w {
                if grid[y * w + x] == SQUARE {
                    if y > 0 {
                        last_free[x] = y - 1;
                    }
                }
                if grid[y * w + x] == ROUND {
                    if last_free[x] > y {
                        // move south
                        grid[last_free[x] * w + x] = ROUND;
                        grid[y * w + x] = EMPTY;
                        last_free[x] -= 1;
                    } else {
                        if y > 0 {
                            last_free[x] = y - 1;
                        }
                    }
                }
            }
        }

        let mut last_free = vec![w - 1; h];

        // TODO: this is direction dependent...
        for x in (0..w).rev() {
            for y in 0..h {
                if grid[y * w + x] == SQUARE {
                    if x > 0 {
                        last_free[y] = x - 1;
                    }
                }
                if grid[y * w + x] == ROUND {
                    if last_free[y] > x {
                        // move east
                        grid[y * w + last_free[y]] = ROUND;
                        grid[y * w + x] = EMPTY;
                        last_free[y] -= 1;
                    } else {
                        if x > 0 {
                            last_free[y] = x - 1;
                        }
                    }
                }
            }
        }
        num += 1;
    }
    let count = grid
        .iter()
        .enumerate()
        .map(|(idx, v)| if *v == ROUND { h - idx / w } else { 0 })
        .sum::<usize>();
    Some(count as u32)
}

fn print_grid(grid: &Vec<u8>, w: usize, h: usize) {
    for row in 0..h {
        for col in 0..w {
            print!(
                "{}",
                match grid[row * w + col] {
                    EMPTY => ".",
                    ROUND => "O",
                    SQUARE => "#",
                    _ => "?",
                }
            );
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
