use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let lines = input.trim().split('\n').collect_vec();
    let mut values: Vec<u32> = Vec::new();
    for row in 0..lines.len() {
        let line = lines[row];
        for cap in re.captures_iter(line) {
            let v = cap.get(1).unwrap();
            let mut valid = false;
            for y in row as i32 - 1..=row as i32 + 1 {
                if y >= 0 && y < lines.len() as i32 {
                    for x in v.start() as i32 - 1..v.end() as i32 + 1 {
                        if x >= 0 && x < line.len() as i32 {
                            let c = lines[y as usize].as_bytes()[x as usize];
                            if c != 46 && (c < 48 || c > 57) {
                                valid = true;
                            }
                        }
                    }
                }
            }
            if valid {
                values.push(v.as_str().parse().unwrap());
            }
        }
    }

    Some(values.iter().sum())
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
