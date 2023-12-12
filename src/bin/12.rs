use std::{cmp::min, collections::HashMap};

use aoc_parse::{parser, prelude::*};
use itertools::Itertools;

advent_of_code::solution!(12);

const EMPTY: usize = 0;
const SPRING: usize = 1;
const WILD: usize = 2;

fn find_arrangements(v: &Vec<usize>, sizes: &[i32]) -> i64 {
    find_arrangements_internal(v, sizes, 0, &mut HashMap::new())
}

fn find_arrangements_internal(
    v: &[usize],
    sizes: &[i32],
    min_space: usize,
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    // If we have processed all the springs, check the rest of the pattern is empty
    if sizes.len() == 0 {
        if v[..].iter().all(|c| *c == EMPTY || *c == WILD) {
            // this is one single successful arrangement
            return 1;
        } else {
            // not a successful arrangement
            return 0;
        }
    }

    // Check if we've already processed the remainder before and reuse that calculation
    // Key by remaining string length and remaining number of spring combinations to process
    // As we go left to right, we know these compose a complete right-hand side
    let memo_key = (v.len(), sizes.len());
    if let Some(result) = memo.get(&memo_key) {
        return *result;
    }

    // Minimum that must be required to complete the remaining springs (total size of
    // the springs, plus a gap of at least one in between)
    let min_required = sizes.iter().sum::<i32>() as usize + sizes.len() - 1;
    // The number of possible spaces at the start, which we can try combinations of
    let extra_spaces_remaining = v.len() - min_required;
    // Index of the first known spring, so we don't attempt any more spaces than that
    let first_spring = v.iter().position(|i| *i == SPRING).unwrap_or(v.len());
    let spaces_to_check = min(extra_spaces_remaining, first_spring);
    // How many spring characters are required to make the next pattern valid
    let expected_spring_size = sizes[0] as usize;

    let mut arrangements = 0;
    // min_space starts as 0, but will be 1 for all interleaving segments
    // check if we have a valid next segment, for each possible number of extra spaces
    // we can use at this point
    for i in min_space..=spaces_to_check {
        if v[0..i].iter().all(|c| *c == EMPTY || *c == WILD)
            && v[i..i + expected_spring_size]
                .iter()
                .all(|c| *c == SPRING || *c == WILD)
        {
            // if the segment is valid, move on to the next segment of spaces and springs, then add the total found
            arrangements +=
                find_arrangements_internal(&v[i + expected_spring_size..], &sizes[1..], 1, memo);
        }
    }

    // memoize this result in case we need the calculation again to improve performace
    memo.insert(memo_key, arrangements);

    arrangements
}

fn parse_map(input: &str) -> Vec<(Vec<usize>, Vec<i32>)> {
    let p = parser!(lines(
        char_of(".#?")+ string(" ") repeat_sep(i32, ",")
    ));

    let v = p.parse(input).unwrap();
    v.into_iter().map(|line| (line.0, line.2)).collect_vec()
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        parse_map(input)
            .iter()
            .map(|line| find_arrangements(&line.0, &line.1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        parse_map(input)
            .iter()
            .map(|line| {
                let mut v = Vec::new();
                let mut sizes = Vec::new();

                for i in 0..5 {
                    v.extend(line.0.iter());
                    sizes.extend(line.1.iter());
                    if i < 4 {
                        v.push(WILD);
                    }
                }

                find_arrangements(&v, &sizes)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_map(str: &str) -> Vec<usize> {
        str.chars()
            .map(|c| match c {
                '.' => EMPTY,
                '#' => SPRING,
                '?' => WILD,
                _ => panic!("Invalid test input {c}")
            })
            .collect_vec()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(find_arrangements(&str_to_map("???.###"), &[1, 1, 3]), 1);
        assert_eq!(
            find_arrangements(&str_to_map(".??..??...?##."), &[1, 1, 3]),
            4
        );
        assert_eq!(
            find_arrangements(&str_to_map("?#?#?#?#?#?#?#?"), &[1, 3, 1, 6]),
            1
        );
        assert_eq!(
            find_arrangements(&str_to_map("????.#...#..."), &[4, 1, 1]),
            1
        );
        assert_eq!(
            find_arrangements(&str_to_map("????.######..#####."), &[1, 6, 5]),
            4
        );
        assert_eq!(
            find_arrangements(&str_to_map("?###????????"), &[3, 2, 1]),
            10
        );

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
