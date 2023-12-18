use std::cmp::min;

use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;

advent_of_code::solution!(3);

fn read_engine_schema<F>(input: &str, mut process: F)
where
    F: FnMut(u8, (usize, usize), &str, (usize, usize)),
{
    let re = Regex::new(r"(\d+)").unwrap();
    let lines = input.trim().split('\n').collect_vec();
    for row in 0..lines.len() {
        let line = lines[row];
        // Get all the numbers in the line
        for cap in re.captures_iter(line) {
            let v = cap.get(1).unwrap();
            // Search the box surrounding the number, with bounds checking
            for y in row.checked_sub(1).unwrap_or(0)..min(row + 2, lines.len()) {
                for x in v.start().checked_sub(1).unwrap_or(0)..min(v.end() + 1, line.len()) {
                    let c = lines[y].as_bytes()[x];
                    process(c, (x, y), v.as_str(), (v.start(), row));
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parts = FxHashMap::default();
    read_engine_schema(input, |c, _, part_num, part_loc| {
        // if the coordinate is a symbol (in this case, not a . or 0-9), add it to the list of parts
        if c != b'.' && (c < b'0' || c > b'9') {
            // add to hashmap with a key for that coordinate to avoid duplicates on that number
            parts.insert(part_loc, part_num.parse().unwrap());
        }
    });

    Some(parts.values().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut gears: FxHashMap<(usize, usize), Vec<u32>> = FxHashMap::default();

    read_engine_schema(input, |c, gear_loc, part_num, _| {
        // if the coordinate is '*', process it as a gear
        if c == b'*' {
            // Append the number to the list of numbers for that gear (creating the list if none found yet)
            let part_num = part_num.parse::<u32>().unwrap();
            gears
                .entry(gear_loc)
                .and_modify(|previous| previous.push(part_num))
                .or_insert(vec![part_num]);
        }
    });

    // For gears with exactly two part numbers, multiply them and then sum the total
    Some(
        gears
            .values()
            .filter(|g| g.len() == 2)
            .map(|g| g[0] * g[1])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
