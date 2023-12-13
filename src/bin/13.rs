use std::cmp::min;

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let p = parser!(sections(
        lines(char_of(".#")+)
    ));
    let v = p.parse(input).unwrap();

    let mut total = 0;
    for section in v {
        // horizontal
        for idx in 0..section.len() - 1 {
            let num = min(idx + 1, section.len() - idx - 1);
            let mut same = true;
            for i in 0..num {
                if section[idx - i] != section[idx + (i + 1)] {
                    same = false;
                    break;
                }
            }
            if same {
                total += (idx + 1) * 100;
                break;
            }
        }

        // vertical
        // TODO: refactor
        for idx in 0..section[0].len() - 1 {
            let num = min(idx + 1, section[0].len() - idx - 1);
            let mut same = true;
            for i in 0..num {
                if !section.iter().all(|row| row[idx - i] == row[idx + (i + 1)]) {
                    same = false;
                    break;
                }
            }
            if same {
                total += idx + 1;
                break;
            }
        }
    }

    Some(total as u32)
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
