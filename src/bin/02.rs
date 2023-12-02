use std::cmp::{max, min};

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter_map(|line| {
                let (game, result) = line.split(':').collect_tuple()?;
                let game_id: u32 = game[5..].parse().unwrap();
                let (mut red, mut green, mut blue) = (0, 0, 0);
                let mut valid = true;
                for draw in result.split(";") {
                    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
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

                println!(
                    "game {game_id}, red {red}, green {green}, blue {blue}, {}",
                    red <= 12 && green <= 13 && blue <= 14
                );

                if valid {
                    Some(game_id)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter_map(|line| {
                let (game, result) = line.split(':').collect_tuple()?;
                let game_id: u32 = game[5..].parse().unwrap();
                let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
                for draw in result.split(";") {
                    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
                    for c in re.captures_iter(draw) {
                        let count: u32 = c.get(1).unwrap().as_str().parse().unwrap();
                        match c.get(2)?.as_str() {
                            "red" => min_red = max(count, min_red),
                            "green" => min_green = max(count, min_green),
                            "blue" => min_blue = max(count, min_blue),
                            _ => (),
                        }
                    }
                }

                let power = min_red * min_green * min_blue;
                println!(
                    "game {game_id}, red {min_red}, green {min_green}, blue {min_blue}, {power}"
                );

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
