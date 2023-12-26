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

pub fn part_two(input: &str) -> Option<i128> {
    let hailstones = parse_input(input);

    // Translate so one passes through the origin to simplify equations
    let (first_pos, first_velocity) = &hailstones[0];
    let translated_hailstones = hailstones[1..]
        .iter()
        .map(|h| {
            let (pos, velocity) = h;
            (
                vec![
                    pos[0] - first_pos[0],
                    pos[1] - first_pos[1],
                    pos[2] - first_pos[2],
                ],
                vec![
                    velocity[0] - first_velocity[0],
                    velocity[1] - first_velocity[1],
                    velocity[2] - first_velocity[2],
                ],
            )
        })
        .collect::<Vec<_>>();

    // Find the plane between the origin and the line of the first hailstone
    // Rock's trajectory will be on that plane

    // First vector = velocity of the next hailstone
    // Second vector = origin and any point on the line (just use the starting point)
    // Plane = cross product
    let (next_hailstone_pos, next_hailstone_velocity) = &translated_hailstones[0];
    let (x1, y1, z1) = (
        next_hailstone_velocity[0],
        next_hailstone_velocity[1],
        next_hailstone_velocity[2],
    );
    let (x2, y2, z2) = (
        next_hailstone_pos[0],
        next_hailstone_pos[1],
        next_hailstone_pos[2],
    );
    // cross product to get normal vector
    let (cp_x, cp_y, cp_z) = (y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2);

    // plane = cp_x * x + cp_y * y + cp_z * z (using origin as a point on the plane), for a point (x, y, z)
    println!("plane = {}x + {}y + {}z = 0", cp_x, cp_y, cp_z);

    // Find intersection between hailstone 3 with the plane and use
    // this to determine the trajectory of the rock
    // intersection: (p.x + v.x * t, p.y + v.y * t, p.z + v.z * t)
    // cp_x * p.x + cp_y * p.y + cp_z * p.z + t * (cp_x * v.x + cp_y * v.y + cp_z * v.z)
    // t = -(cp_x * p.x + cp_y * p.y + cp_z * p.z) / (cp_x * v.x + cp_y * v.y + cp_z * v.z)
    let (p, v) = &translated_hailstones[1];
    let t3 = -(cp_x * p[0] + cp_y * p[1] + cp_z * p[2]) / (cp_x * v[0] + cp_y * v[1] + cp_z * v[2]);
    let intersection_3 = vec![p[0] + v[0] * t3, p[1] + v[1] * t3, p[2] + v[2] * t3];
    // calculate time for the previous hailstone
    let (p, v) = &translated_hailstones[2];
    let t2 = -(cp_x * p[0] + cp_y * p[1] + cp_z * p[2]) / (cp_x * v[0] + cp_y * v[1] + cp_z * v[2]);
    let intersection_2 = vec![p[0] + v[0] * t2, p[1] + v[1] * t2, p[2] + v[2] * t2];

    let dt = t3 - t2;
    let v = vec![
        (intersection_3[0] - intersection_2[0]) / dt,
        (intersection_3[1] - intersection_2[1]) / dt,
        (intersection_3[2] - intersection_2[2]) / dt,
    ];

    println!(
        "Intersection points {:?} and {:?} gives vector {:?}",
        intersection_2, intersection_3, v
    );

    // Rock line will be:
    // x = start.x + v.x * t
    // y = start.y + v.y * t
    // z = start.z + v.z * t
    // therefore:
    // start.x = x - v.x * t
    // start.y = y - v.y * t
    // start.z = z - v.z * t
    let start = vec![
        intersection_2[0] - v[0] * t2,
        intersection_2[1] - v[1] * t2,
        intersection_2[2] - v[2] * t2,
    ];

    println!("Start is {:?}", start);

    // Put back into original frame of reference
    let start = vec![
        start[0] + first_pos[0],
        start[1] + first_pos[1],
        start[2] + first_pos[2],
    ];

    println!("Start in original frame of reference {:?}", start);

    Some(start[0] + start[1] + start[2])
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
        assert_eq!(result, Some(47));
    }
}
