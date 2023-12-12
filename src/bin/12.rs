use std::cmp::min;

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(12);

fn find_arrangements(str: &str, sizes: &[i32]) -> i32 {
    find_arrangements_internal(str, sizes, 0)
}

fn find_arrangements_internal(str: &str, sizes: &[i32], min_space: usize) -> i32 {
    if sizes.len() == 0 {
        let b = str.as_bytes();
        if b[..].iter().all(|c| *c == b'.' || *c == b'?') {
            return 1;
        } else {
            return 0;
        }
    }

    let min_required = sizes.iter().sum::<i32>() + sizes.len() as i32 - 1;
    let to_allocate = str.len() as i32 - min_required;
    let first_spring = str.find('#').unwrap_or(str.len()) as i32;
    let check_to = min(to_allocate, first_spring) as usize;
    let expected = sizes[0] as usize;

    let b = str.as_bytes();
    let mut arrangements = 0;
    for i in min_space..=check_to {
        if b[0..i].iter().all(|c| *c == b'.' || *c == b'?')
            && b[i..i + expected].iter().all(|c| *c == b'#' || *c == b'?')
        {
            arrangements += find_arrangements_internal(&str[i + expected..], &sizes[1..], 1);
        }
    }

    arrangements
}

pub fn part_one(input: &str) -> Option<i32> {
    let p = parser!(lines(
        // char_of(".#?")+ string(" ") repeat_sep(i32, ",")
        string(any_char+) string(" ") repeat_sep(i32, ",")
    ));

    let v = p.parse(input).unwrap();

    Some(
        v.iter()
            .map(|line| find_arrangements(line.0.as_str(), line.2.as_slice()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let p = parser!(lines(
        // char_of(".#?")+ string(" ") repeat_sep(i32, ",")
        string(any_char+) string(" ") repeat_sep(i32, ",")
    ));

    let v = p.parse(input).unwrap();

    Some(
        v.iter()
            .map(|line| {
                let mut s = String::new();
                let mut info = Vec::new();

                for i in 0..5 {
                    s += &line.0;
                    for e in &line.2 {
                        info.push(*e);
                    }
                    if i < 4 {
                        s += "?";
                    }
                }

                println!("Checking {} {:?}", s, info);
                find_arrangements(s.as_str(), &info)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(find_arrangements("???.###", &[1, 1, 3]), 1);
        assert_eq!(find_arrangements(".??..??...?##.", &[1, 1, 3]), 4);
        assert_eq!(find_arrangements("?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
        assert_eq!(find_arrangements("????.#...#...", &[4, 1, 1]), 1);
        assert_eq!(find_arrangements("????.######..#####.", &[1, 6, 5]), 4);
        assert_eq!(find_arrangements("?###????????", &[3, 2, 1]), 10);

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
