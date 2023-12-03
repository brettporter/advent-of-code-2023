use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

fn read_engine_schema<F>(input: &str, mut process: F)
where
    F: FnMut(u8, (i32, i32), &str, (usize, usize)),
{
    let re = Regex::new(r"(\d+)").unwrap();
    let lines = input.trim().split('\n').collect_vec();
    for row in 0..lines.len() {
        let line = lines[row];
        // Get all the numbers in the line
        for cap in re.captures_iter(line) {
            let v = cap.get(1).unwrap();
            // Search the box surrounding the number, with bounds checking
            for y in row as i32 - 1..=row as i32 + 1 {
                if y >= 0 && y < lines.len() as i32 {
                    for x in v.start() as i32 - 1..v.end() as i32 + 1 {
                        if x >= 0 && x < line.len() as i32 {
                            let c = lines[y as usize].as_bytes()[x as usize];
                            process(c, (x, y), v.as_str(), (v.start(), row));
                        }
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parts = HashMap::new();
    read_engine_schema(input, |c, _, v, key| {
        // if the coordinate is a symbol (in this case, not a . or 0-9), add it to the list of parts
        if c != b'.' && (c < b'0' || c > b'9') {
            // add to hashmap with a key for that coordinate to avoid duplicates on that number
            parts.insert(key, v.parse().unwrap());
        }
    });

    Some(parts.values().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    read_engine_schema(input, |c, key, v, _| {
        // if the coordinate is '*', process it as a gear
        if c == b'*' {
            // Append the number to the list of numbers for that gear (creating the list if none found yet)
            if let Some(previous) = gears.get_mut(&key) {
                previous.push(v.parse::<u32>().unwrap());
            } else {
                gears.insert(key, vec![v.parse::<u32>().unwrap()]);
            }
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
