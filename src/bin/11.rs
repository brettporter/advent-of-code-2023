use aoc_parse::{parser, prelude::*};
use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

fn calculate_distance(start: &Point, end: &Point) -> isize {
    (end.x - start.x).abs() + (end.y - start.y).abs()
}

fn parse_map(input: &str) -> Vec<Point> {
    let p = parser!(lines(
        char_of(".#")+
    ));
    let v = p.parse(input).unwrap();

    let mut result = Vec::new();
    for (y, row) in v.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 1 {
                result.push(Point::new(x as isize, y as isize));
            }
        }
    }
    result
}

fn expand_map(input: &Vec<Point>, expansion_rate: isize) -> Vec<Point> {
    let cols = input.iter().map(|p| p.x).collect_vec();
    let rows = input.iter().map(|p| p.y).collect_vec();

    let w = cols.iter().max().unwrap() + 1;
    let h = rows.iter().max().unwrap() + 1;

    let expand_cols = (0..w).filter(|col| !cols.contains(col)).collect_vec();
    let expand_rows = (0..h).filter(|row| !rows.contains(row)).collect_vec();

    input
        .iter()
        .map(|p| {
            Point::new(
                p.x + expand_cols.iter().filter(|col| **col < p.x).count() as isize
                    * (expansion_rate - 1),
                p.y + expand_rows.iter().filter(|row| **row < p.y).count() as isize
                    * (expansion_rate - 1),
            )
        })
        .collect_vec()
}

fn calculate_total_distance(map: Vec<Point>) -> Option<isize> {
    Some(
        map.into_iter()
            .combinations(2)
            .map(|v| calculate_distance(&v[0], &v[1]))
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<isize> {
    let map = expand_map(&parse_map(input), 2);
    calculate_total_distance(map)
}

pub fn part_two(input: &str) -> Option<isize> {
    let map = expand_map(&parse_map(input), 1000000);
    calculate_total_distance(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        assert_eq!(calculate_distance(&Point::new(1, 6), &Point::new(5, 11)), 9);
        assert_eq!(
            calculate_distance(&Point::new(4, 0), &Point::new(9, 10)),
            15
        );
        assert_eq!(
            calculate_distance(&Point::new(0, 2), &Point::new(12, 7)),
            17
        );
        assert_eq!(
            calculate_distance(&Point::new(0, 11), &Point::new(5, 11)),
            5
        );
    }

    #[test]
    fn test_parse_map() {
        let map = parse_map(&advent_of_code::template::read_file("examples", DAY));

        let expected = vec![
            Point::new(3, 0),
            Point::new(7, 1),
            Point::new(0, 2),
            Point::new(6, 4),
            Point::new(1, 5),
            Point::new(9, 6),
            Point::new(7, 8),
            Point::new(0, 9),
            Point::new(4, 9),
        ];
        assert_eq!(map, expected);
    }

    #[test]
    fn test_expand_universe() {
        let original_map = parse_map(&advent_of_code::template::read_file("examples", DAY));
        let expected_map = parse_map(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));

        assert_eq!(expand_map(&original_map, 2), expected_map);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let map = parse_map(&advent_of_code::template::read_file("examples", DAY));
        let result = calculate_total_distance(expand_map(&map, 10));
        assert_eq!(result, Some(1030));

        let result = calculate_total_distance(expand_map(&map, 100));
        assert_eq!(result, Some(8410));
    }
}
