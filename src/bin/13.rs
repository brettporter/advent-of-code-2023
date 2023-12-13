use std::cmp::min;

use aoc_parse::{parser, prelude::*};
use itertools::Itertools;

advent_of_code::solution!(13);

fn find_mirror(section: Vec<usize>, error_target: usize) -> Option<u32> {
    let h = section.len();
    // Go through every row and check if mirroring at that row works with an exact number of errors
    for row_idx in 0..h - 1 {
        // Find the shortest side of the mirror
        let num = min(row_idx + 1, h - row_idx - 1);
        let mut errors = 0;
        // Go through every pair of rows starting at the origin
        'outer: for i in 0..num {
            // Count the number of mismatches in these two rows
            let mut diff = section[row_idx - i] ^ (section[row_idx + (i + 1)]);
            while diff > 0 {
                diff &= diff - 1;
                errors += 1;
                if errors > error_target {
                    break 'outer;
                }
            }
        }
        // Make sure we found the right number of smudges
        if errors == error_target {
            return Some(row_idx as u32 + 1);
        }
    }
    None
}

fn process(input: &str, error_target: usize) -> u32 {
    let p = parser!(sections(
        lines(char_of(".#")+)
    ));
    let v = p.parse(input).unwrap();

    let mut total = 0;

    for section in v {
        // Create horizontal rows as a bitmap
        let horizontal = section
            .iter()
            .map(|row| {
                let mut value = 0;
                for i in 0..row.len() {
                    value |= row[i] << i;
                }
                value
            })
            .collect_vec();

        // Create vertical rows as a bitmap (transposed)
        let vertical = (0..section[0].len())
            .map(|col| {
                let mut value = 0;
                for (i, row) in section.iter().enumerate() {
                    value |= row[col] << i;
                }
                value
            })
            .collect_vec();

        if let Some(result) = find_mirror(vertical, error_target) {
            total += result;
        } else {
            if let Some(result) = find_mirror(horizontal, error_target) {
                total += result * 100;
            }
        }
    }
    total as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(process(input, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(process(input, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
