use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i128, newline, space0},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<usize> {
    count_intersections(input, 200000000000000.0, 400000000000000.0)
}

fn count_intersections(input: &str, min: f64, max: f64) -> Option<usize> {
    let hailstones = parse_input(input);

    Some(
        hailstones
            .iter()
            .combinations(2)
            .filter(|v| check_intersection(v[0], v[1], min, max))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<(Vec<i128>, Vec<i128>)> {
    fn parse_trajectory(input: &str) -> IResult<&str, (Vec<i128>, Vec<i128>)> {
        let (input, (pos, _, velocity)) = tuple((
            separated_list1(tag(", "), i128),
            tag(" @ "),
            separated_list1(tag(", "), preceded(space0, i128)),
        ))(input)?;

        Ok((input, (pos, velocity)))
    }

    let (_, hailstones) = many1(terminated(parse_trajectory, newline))(input).unwrap();
    hailstones
}

fn check_intersection(
    (pos1, vel1): &(Vec<i128>, Vec<i128>),
    (pos2, vel2): &(Vec<i128>, Vec<i128>),
    min: f64,
    max: f64,
) -> bool {
    let (x1, y1, x2, y2) = (pos1[0], pos1[1], pos1[0] + vel1[0], pos1[1] + vel1[1]);
    let (a1, b1) = (y2 - y1, x1 - x2);
    let c1 = a1 * x1 + b1 * y1;

    let (x1, y1, x2, y2) = (pos2[0], pos2[1], pos2[0] + vel2[0], pos2[1] + vel2[1]);
    let (a2, b2) = (y2 - y1, x1 - x2);
    let c2 = a2 * x1 + b2 * y1;

    let d = a1 * b2 - a2 * b1;
    if d == 0 {
        // parallel
        false
    } else {
        let int_x = (b2 * c1 - b1 * c2) as f64 / d as f64;
        let int_y = (a1 * c2 - a2 * c1) as f64 / d as f64;
        // TODO: simplify
        if (vel1[0].signum() < 0 && int_x > pos1[0] as f64)
            || (vel1[0].signum() > 0 && int_x < pos1[0] as f64)
            || (vel1[1].signum() < 0 && int_y > pos1[1] as f64)
            || (vel1[1].signum() > 0 && int_y < pos1[1] as f64)
            || (vel2[0].signum() < 0 && int_x > pos2[0] as f64)
            || (vel2[0].signum() > 0 && int_x < pos2[0] as f64)
            || (vel2[1].signum() < 0 && int_y > pos2[1] as f64)
            || (vel2[1].signum() > 0 && int_y < pos2[1] as f64)
        {
            false
        } else {
            int_x >= min && int_x <= max && int_y >= min && int_y <= max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_intersection() {
        // TODO: table tests?

        // Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).
        assert!(check_intersection(
            &(vec![19, 13, 30], vec![-2, 1, -2]),
            &(vec![18, 19, 22], vec![-1, -1, -2]),
            7.0,
            27.0
        ));

        // Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).
        assert!(check_intersection(
            &(vec![19, 13, 30], vec![-2, 1, -2]),
            &(vec![20, 25, 34], vec![-2, -2, -4]),
            7.0,
            27.0
        ));

        // Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).
        assert!(
            check_intersection(
                &(vec![19, 13, 30], vec![-2, 1, -2]),
                &(vec![12, 31, 28], vec![-1, -2, -1]),
                7.0,
                27.0
            ) == false
        );

        // Hailstones' paths crossed in the past for hailstone A.
        assert!(
            check_intersection(
                &(vec![19, 13, 30], vec![-2, 1, -2]),
                &(vec![20, 19, 15], vec![1, -5, -3]),
                7.0,
                27.0
            ) == false
        );

        // Hailstones' paths are parallel; they never intersect.
        assert!(
            check_intersection(
                &(vec![18, 19, 22], vec![-1, -1, -2]),
                &(vec![20, 25, 34], vec![-2, -2, -4]),
                7.0,
                27.0
            ) == false
        );

        // Hailstones' paths will cross outside the test area (at x=-6, y=-5).
        assert!(
            check_intersection(
                &(vec![18, 19, 22], vec![-1, -1, -2]),
                &(vec![12, 31, 28], vec![-1, -2, -1]),
                7.0,
                27.0
            ) == false
        );

        // Hailstones' paths crossed in the past for both hailstones.
        assert!(
            check_intersection(
                &(vec![18, 19, 22], vec![-1, -1, -2]),
                &(vec![20, 19, 15], vec![1, -5, -3]),
                7.0,
                27.0
            ) == false
        );

        // Hailstones' paths will cross outside the test area (at x=-2, y=3).
        assert!(
            check_intersection(
                &(vec![20, 25, 34], vec![-2, -2, -4]),
                &(vec![12, 31, 28], vec![-1, -2, -1]),
                7.0,
                27.0
            ) == false
        );

        // Hailstones' paths crossed in the past for hailstone B.
        assert!(
            check_intersection(
                &(vec![20, 25, 34], vec![-2, -2, -4]),
                &(vec![20, 19, 15], vec![1, -5, -3]),
                7.0,
                27.0
            ) == false
        );

        // Hailstones' paths crossed in the past for both hailstones.
        assert!(
            check_intersection(
                &(vec![12, 31, 28], vec![-1, -2, -1]),
                &(vec![20, 19, 15], vec![1, -5, -3]),
                7.0,
                27.0
            ) == false
        );
    }

    #[test]
    fn test_part_one() {
        let result = count_intersections(
            &advent_of_code::template::read_file("examples", DAY),
            7.0,
            27.0,
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
