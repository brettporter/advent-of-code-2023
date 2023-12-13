use std::cmp::min;

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    Some(process(input, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(process(input, 1))
}

fn process(input: &str, error_target: usize) -> u32 {
    let p = parser!(sections(
        lines(char_of(".#")+)
    ));
    let v = p.parse(input).unwrap();

    let mut total = 0;
    'outer: for section in v {
        // horizontal
        let h = section.len();
        // Go through every row and check if mirroring at that row works with an exact number of errors
        for row_idx in 0..h - 1 {
            // Find the shortest side of the mirror
            let num = min(row_idx + 1, h - row_idx - 1);
            let mut errors = 0;
            // Go through every pair of rows starting at the origin
            for i in 0..num {
                // Count the number of mismatches in these two rows
                let (line1, line2) = (&section[row_idx - i], &section[row_idx + (i + 1)]);
                errors += (0..line1.len()).filter(|j| line1[*j] != line2[*j]).count();
                // If we haven't exceeded the number of errors, keep going
                if errors > error_target {
                    break;
                }
            }
            // Make sure we found the right number of smudges
            if errors == error_target {
                total += (row_idx + 1) * 100;
                continue 'outer;
            }
        }

        // vertical
        let w = section[0].len();
        // Go through every column and check if mirroring at that column works with an exact number of errors
        for col_idx in 0..w - 1 {
            // Find the shortest side of the mirror
            let num = min(col_idx + 1, w - col_idx - 1);
            let mut errors = 0;
            // Go through every pair of columns starting at the origin
            for i in 0..num {
                // Count the number of mismatches in these two columns
                errors += section
                    .iter()
                    .filter(|row| row[col_idx - i] != row[col_idx + (i + 1)])
                    .count();
                // If we haven't exceeded the number of errors, keep going
                if errors > error_target {
                    break;
                }
            }
            // Make sure we found the right number of smudges
            if errors == error_target {
                total += col_idx + 1;
                break;
            }
        }
    }
    total as u32
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
