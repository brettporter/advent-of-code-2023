use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(14);

const EMPTY: usize = 0;
const SQUARE: usize = 1;
const ROUND: usize = 2;

pub fn part_one(input: &str) -> Option<u32> {
    let p = parser!(lines(
        char_of(".#O")+
    ));

    let mut grid = p.parse(input).unwrap();
    // let direction = (0, -1);

    let w = grid[0].len();
    let h = grid.len();
    let mut last_free = vec![0; w];

    // TODO: this is direction dependent...
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == SQUARE {
                last_free[x] = y + 1;
            }
            if grid[y][x] == ROUND {
                if last_free[x] < y {
                    // move up
                    grid[last_free[x]][x] = ROUND;
                    count += h - last_free[x];

                    grid[y][x] = EMPTY;
                    last_free[x] += 1;
                } else {
                    count += h - y;
                    last_free[x] = y + 1;
                }
            }
        }
    }

    Some(count as u32)
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
