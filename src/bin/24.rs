use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i128, newline, space0},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

advent_of_code::solution!(24);

fn count_intersections(input: &str, min: i128, max: i128) -> Option<usize> {
    let hailstones = parse_input(input);

    Some(
        hailstones
            .iter()
            .combinations(2)
            .filter(|v| check_intersection(v[0], v[1], min, max))
            .count(),
    )
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    fn parse_trajectory(input: &str) -> IResult<&str, Hailstone> {
        let (input, (pos, _, velocity)) = tuple((
            separated_list1(tag(", "), i128),
            tag(" @ "),
            separated_list1(tag(", "), preceded(space0, i128)),
        ))(input)?;

        Ok((input, Hailstone::from(pos, velocity)))
    }

    let (_, hailstones) = many1(terminated(parse_trajectory, newline))(input).unwrap();
    hailstones
}

#[derive(Debug)]
struct Coordinate {
    x: i128,
    y: i128,
    z: i128,
}

impl Coordinate {
    fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }

    fn diff(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn cross_product(&self, other: &Coordinate) -> Coordinate {
        // cross product to get normal vector
        Coordinate::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn div(&self, dt: i128) -> Self {
        Self {
            x: self.x / dt,
            y: self.y / dt,
            z: self.z / dt,
        }
    }
}

type Point = Coordinate;
type Velocity = Coordinate;

struct Hailstone {
    pos: Point,
    velocity: Velocity,
}

impl Hailstone {
    fn new(pos: Point, velocity: Velocity) -> Self {
        Self { pos, velocity }
    }

    fn from(pos: Vec<i128>, velocity: Vec<i128>) -> Self {
        Self {
            pos: Point {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            },
            velocity: Velocity {
                x: velocity[0],
                y: velocity[1],
                z: velocity[2],
            },
        }
    }

    fn intersection_with_plane(&self, cp: &Coordinate) -> (i128, Point) {
        // Solve for t by substituting this into the equation of the plane:
        // hailstone line equation
        //   (p.x + v.x * t, p.y + v.y * t, p.z + v.z * t)
        // substituded into plane and collect like terms:
        //   cp_x * p.x + cp_y * p.y + cp_z * p.z + t * (cp_x * v.x + cp_y * v.y + cp_z * v.z)
        // solve for t:
        //   t = -(cp_x * p.x + cp_y * p.y + cp_z * p.z) / (cp_x * v.x + cp_y * v.y + cp_z * v.z)
        let (p, v) = (&self.pos, &self.velocity);
        let t = -(cp.x * p.x + cp.y * p.y + cp.z * p.z) / (cp.x * v.x + cp.y * v.y + cp.z * v.z);
        let intersection = Point::new(p.x + v.x * t, p.y + v.y * t, p.z + v.z * t);
        (t, intersection)
    }
}

fn check_intersection(
    hailstone1: &Hailstone,
    hailstone2: &Hailstone,
    min: i128,
    max: i128,
) -> bool {
    // velocity is the vector form of the line
    // we can represent both lines as an equation and solve them simultaneously
    // to find the x, y that fits both
    let (pos1, vel1) = (&hailstone1.pos, &hailstone1.velocity);
    let c1 = vel1.y * pos1.x - vel1.x * pos1.y;

    let (pos2, vel2) = (&hailstone2.pos, &hailstone2.velocity);
    let c2 = vel2.y * pos2.x - vel2.x * pos2.y;

    let d = vel2.y * vel1.x - vel1.y * vel2.x;
    if d == 0 {
        // parallel lines
        return false;
    }

    // find the intersection point
    let int_x = (vel1.x * c2 - vel2.x * c1) / d;
    let int_y = (vel1.y * c2 - vel2.y * c1) / d;
    // check signum to determine which side of the starting position the intersection
    // needs to be on to avoid intersections that happen in the past
    // TODO: can we solve for t on a parametric equation instead?
    if (vel1.x.signum() > 0 || int_x < pos1.x)
        && (vel1.x.signum() < 0 || int_x > pos1.x)
        && (vel1.y.signum() > 0 || int_y < pos1.y)
        && (vel1.y.signum() < 0 || int_y > pos1.y)
        && (vel2.x.signum() > 0 || int_x < pos2.x)
        && (vel2.x.signum() < 0 || int_x > pos2.x)
        && (vel2.y.signum() > 0 || int_y < pos2.y)
        && (vel2.y.signum() < 0 || int_y > pos2.y)
    {
        // determine if the intersection point is within the boundary defined
        return int_x >= min && int_x <= max && int_y >= min && int_y <= max;
    }
    return false;
}

pub fn part_one(input: &str) -> Option<usize> {
    count_intersections(input, 200_000_000_000_000, 400_000_000_000_000)
}

pub fn part_two(input: &str) -> Option<i128> {
    let hailstones = parse_input(input);

    // To find the path of the rock, we move all the hailstones into the frame of
    // reference of the first hailstone. This means it will be stationary at the origin
    // which simplifies the equations.
    //
    // We then use a second hailstone to define a plane with the first hailstone at the
    // origin, which will give a plane for all possible trajectories of the rock.
    //
    // Finally, find the intersection points of two other hailstones with the plane.
    // These will be specific collisions with the rock that we can map its trajectory,
    // and the distance between them will define the velocity of the rock that can be
    // extrapolated to give the start point.

    // Translate hailstones into the frame of reference of the first
    let first_hailstone = &hailstones[0];
    let translated_hailstones = hailstones[1..]
        .iter()
        .map(|h| {
            Hailstone::new(
                h.pos.diff(&first_hailstone.pos),
                h.velocity.diff(&first_hailstone.velocity),
            )
        })
        .collect::<Vec<_>>();

    // Find the plane between the origin and the line of the first hailstone
    // First vector = velocity of the next hailstone
    // Second vector = origin and any point on the line of that hailstone (we use the starting point)
    // Taking the cross product gives the normal vector, which can be used to represent the plane
    // cp = (A, B, C) then plane is Ax + By + Cz = 0 (no additional constant due to being through the origin)
    let next_hailstone = &translated_hailstones[0];
    let cp = next_hailstone.velocity.cross_product(&next_hailstone.pos);

    // Find intersection between two other hailstones and the plane, used to
    // determine the trajectory of the rock
    let (t1, intersection1) = translated_hailstones[1].intersection_with_plane(&cp);
    let (t2, intersection2) = translated_hailstones[2].intersection_with_plane(&cp);

    // Find the amount of time between the two intersections and then normalise the vector accordingly
    let dt = t2 - t1;
    let v = intersection2.diff(&intersection1).div(dt);

    // Rock line will be:
    //   x = start.x + v.x * t
    //   y = start.y + v.y * t
    //   z = start.z + v.z * t
    // therefore to find the starting position:
    //   start.x = x - v.x * t
    //   start.y = y - v.y * t
    //   start.z = z - v.z * t
    let start = Point::new(
        intersection1.x - v.x * t1,
        intersection1.y - v.y * t1,
        intersection1.z - v.z * t1,
    );

    // Put back into original frame of reference
    let start = start.add(&first_hailstone.pos);

    Some(start.x + start.y + start.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_intersection() {
        // TODO: table tests?

        // Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).
        assert!(check_intersection(
            &Hailstone::from(vec![19, 13, 30], vec![-2, 1, -2]),
            &Hailstone::from(vec![18, 19, 22], vec![-1, -1, -2]),
            7,
            27
        ));

        // Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).
        assert!(check_intersection(
            &Hailstone::from(vec![19, 13, 30], vec![-2, 1, -2]),
            &Hailstone::from(vec![20, 25, 34], vec![-2, -2, -4]),
            7,
            27
        ));

        // Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).
        assert!(
            check_intersection(
                &Hailstone::from(vec![19, 13, 30], vec![-2, 1, -2]),
                &Hailstone::from(vec![12, 31, 28], vec![-1, -2, -1]),
                7,
                27
            ) == false
        );

        // Hailstones' paths crossed in the past for hailstone A.
        assert!(
            check_intersection(
                &Hailstone::from(vec![19, 13, 30], vec![-2, 1, -2]),
                &Hailstone::from(vec![20, 19, 15], vec![1, -5, -3]),
                7,
                27
            ) == false
        );

        // Hailstones' paths are parallel; they never intersect.
        assert!(
            check_intersection(
                &Hailstone::from(vec![18, 19, 22], vec![-1, -1, -2]),
                &Hailstone::from(vec![20, 25, 34], vec![-2, -2, -4]),
                7,
                27
            ) == false
        );

        // Hailstones' paths will cross outside the test area (at x=-6, y=-5).
        assert!(
            check_intersection(
                &Hailstone::from(vec![18, 19, 22], vec![-1, -1, -2]),
                &Hailstone::from(vec![12, 31, 28], vec![-1, -2, -1]),
                7,
                27
            ) == false
        );

        // Hailstones' paths crossed in the past for both hailstones.
        assert!(
            check_intersection(
                &Hailstone::from(vec![18, 19, 22], vec![-1, -1, -2]),
                &Hailstone::from(vec![20, 19, 15], vec![1, -5, -3]),
                7,
                27
            ) == false
        );

        // Hailstones' paths will cross outside the test area (at x=-2, y=3).
        assert!(
            check_intersection(
                &Hailstone::from(vec![20, 25, 34], vec![-2, -2, -4]),
                &Hailstone::from(vec![12, 31, 28], vec![-1, -2, -1]),
                7,
                27
            ) == false
        );

        // Hailstones' paths crossed in the past for hailstone B.
        assert!(
            check_intersection(
                &Hailstone::from(vec![20, 25, 34], vec![-2, -2, -4]),
                &Hailstone::from(vec![20, 19, 15], vec![1, -5, -3]),
                7,
                27
            ) == false
        );

        // Hailstones' paths crossed in the past for both hailstones.
        assert!(
            check_intersection(
                &Hailstone::from(vec![12, 31, 28], vec![-1, -2, -1]),
                &Hailstone::from(vec![20, 19, 15], vec![1, -5, -3]),
                7,
                27
            ) == false
        );
    }

    #[test]
    fn test_part_one() {
        let result =
            count_intersections(&advent_of_code::template::read_file("examples", DAY), 7, 27);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
