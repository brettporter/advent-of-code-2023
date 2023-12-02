use std::cmp::max;

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
    Some(
        // For each game line, determine which are valid and then sum their IDs together
        input
            .split('\n')
            .filter_map(|line| {
                let (game, result) = line.split(':').collect_tuple()?;
                let mut valid = true;
                for draw in result.split(";") {
                    let (mut red, mut green, mut blue) = (0, 0, 0);
                    for c in re.captures_iter(draw) {
                        let count: u32 = c.get(1).unwrap().as_str().parse().unwrap();
                        match c.get(2)?.as_str() {
                            "red" => red = count,
                            "green" => green = count,
                            "blue" => blue = count,
                            _ => (),
                        }
                    }

                    if red > 12 || green > 13 || blue > 14 {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    let game_id: u32 = game[5..].parse().unwrap();
                    Some(game_id)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
    Some(
        // For each game line, find the maximum of each colour drawn at a time, then multiply these to get the power and sum the results
        input
            .split('\n')
            .filter_map(|line| {
                let (mut red_required, mut green_required, mut blue_required) = (0, 0, 0);
                for c in re.captures_iter(line) {
                    let count: u32 = c.get(1).unwrap().as_str().parse().unwrap();
                    match c.get(2)?.as_str() {
                        "red" => red_required = max(count, red_required),
                        "green" => green_required = max(count, green_required),
                        "blue" => blue_required = max(count, blue_required),
                        _ => (),
                    }
                }

                let power = red_required * green_required * blue_required;
                Some(power)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
