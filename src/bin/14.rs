use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use itertools::{Either, Itertools};
use seahash::hash;

advent_of_code::solution!(14);

const EMPTY: u8 = 0;
const SQUARE: u8 = 1;
const ROUND: u8 = 2;

enum Direction {
    N,
    W,
    S,
    E,
}

struct Grid {
    w: i32,
    h: i32,
    grid: Vec<u8>,
}

impl Grid {
    fn rock_and_roll(&mut self, direction: Direction) {
        // roll along grid height for N/S, grid width for E/W
        // the other axis is what we scan across to process for each roll step
        let (scan_length, roll_length) = match direction {
            Direction::N | Direction::S => (self.w, self.h),
            Direction::E | Direction::W => (self.h, self.w),
        };

        // We step along from the destination of the roll, so
        // N/W start at 0 and move positively, S/E start at end and move negavtively
        let (start_value, inc) = match direction {
            Direction::N | Direction::W => (0, 1),
            Direction::S | Direction::E => (roll_length - 1, -1),
        };

        // Find the index into the byte array based on the direction
        fn get_index(direction: &Direction, size: i32, scan_idx: i32, roll_idx: i32) -> usize {
            match direction {
                Direction::N | Direction::S => (roll_idx * size + scan_idx) as usize,
                Direction::E | Direction::W => (scan_idx * size + roll_idx) as usize,
            }
        }

        // Create a range iterator based on the direction we are rolling
        fn get_range(inc: i32, roll_length: i32) -> impl Iterator<Item = i32> {
            if inc > 0 {
                Either::Left(0..roll_length)
            } else {
                Either::Right((0..roll_length).rev())
            }
        }

        // Keep track of the point a round rock could roll to
        let mut last_free = vec![start_value; scan_length as usize];

        // Start at the destination for each column and:
        //  if we find an obstacle, set that as the new roll destination for any rocks found
        //  if we find a round rock, roll it to the roll destination and set the new destination above it
        //  if we find a round rock, but nowhere for it to roll to, just set the new destination above it
        for roll_idx in get_range(inc, roll_length) {
            for scan_idx in 0..scan_length {
                let grid_idx = get_index(&direction, scan_length, scan_idx, roll_idx);
                if self.grid[grid_idx] == SQUARE {
                    last_free[scan_idx as usize] = roll_idx + inc;
                }
                if self.grid[grid_idx] == ROUND {
                    if (inc > 0 && last_free[scan_idx as usize] < roll_idx)
                        || (inc < 0 && last_free[scan_idx as usize] > roll_idx)
                    {
                        self.grid[get_index(
                            &direction,
                            scan_length,
                            scan_idx,
                            last_free[scan_idx as usize],
                        )] = ROUND;
                        self.grid[grid_idx] = EMPTY;
                        last_free[scan_idx as usize] = last_free[scan_idx as usize] + inc;
                    } else {
                        last_free[scan_idx as usize] = roll_idx + inc;
                    }
                }
            }
        }
    }

    fn calculate_load(&self) -> i32 {
        // For each round rock, find the line it is on (idx / w)
        // and calculate the number of lines from north (h - value)
        // Return the sum of all rocks
        self.grid
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                if *v == ROUND {
                    self.h - idx as i32 / self.w
                } else {
                    0
                }
            })
            .sum::<i32>()
    }

    fn _print_grid(&self) {
        for row in 0..self.h {
            for col in 0..self.w {
                print!(
                    "{}",
                    match self.grid[(row * self.w + col) as usize] {
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

    fn parse_grid(input: &str) -> Self {
        let p = parser!(lines(
            char_of(".#O")+
        ));

        let map = p.parse(input).unwrap();

        Self {
            w: map[0].len() as i32,
            h: map.len() as i32,
            grid: map.concat().into_iter().map(|b| b as u8).collect_vec(), // convert into a byte array
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::parse_grid(input);
    grid.rock_and_roll(Direction::N);
    Some(grid.calculate_load() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::parse_grid(input);
    let mut seen = HashMap::new();

    let mut target = 1000000000;
    let mut num = 0;
    while num < target {
        // hash the current grid as a byte array and store in a hashmap
        let k = hash(grid.grid.as_slice());
        if let Some(prev) = seen.insert(k, num) {
            // If we found one that was previously encountered, there is a cycle
            // Measure the interval of the cycle
            let interval = num - prev;
            // Find how many iterations would be left to do if we repeated this
            // cycle to the last one before the target
            let remaining = (target - num) % interval;
            // set the target to the same point within the current cycle to short circuit
            target = num + remaining;
        }

        grid.rock_and_roll(Direction::N);
        grid.rock_and_roll(Direction::W);
        grid.rock_and_roll(Direction::S);
        grid.rock_and_roll(Direction::E);

        num += 1;
    }
    Some(grid.calculate_load() as u32)
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
