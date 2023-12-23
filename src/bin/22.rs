use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::FxHashMap;

advent_of_code::solution!(22);

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

const SIZE: i32 = 10;
const Z_SIZE: i32 = 1000;

impl Point {
    fn from_str(s: &str) -> Self {
        let (x, y, z) = s
            .split(",")
            .map(|v| i32::from_str_radix(v, 10).unwrap())
            .collect_tuple()
            .unwrap();
        assert!(x >= 0 && y >= 0 && z > 0 && x < SIZE && y < SIZE && z < Z_SIZE);
        Self { x, y, z }
    }

    fn diff(&self, p: &Point) -> (i32, i32, i32) {
        (self.x - p.x, self.y - p.y, self.z - p.z)
    }

    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Brick {
    id: usize,
    cubes: Vec<Point>,
}

impl Brick {
    fn new(id: usize, start: Point, end: Point) -> Self {
        Self {
            id,
            cubes: Brick::create_cubes(&start, &end),
        }
    }

    fn create_cubes(start: &Point, end: &Point) -> Vec<Point> {
        // Create a Point for each cube to easily compare against the grid
        // Since each is a single straight line, this generalised solution will
        // walk along the direction of whichever is nonzero
        let (dx, dy, dz) = end.diff(start);
        let brick_len = (dx + dy + dz).abs() + 1;
        let (dir_x, dir_y, dir_z) = (dx.signum(), dy.signum(), dz.signum());

        (0..brick_len)
            .into_iter()
            .map(|i| {
                Point::new(
                    start.x + i * dir_x,
                    start.y + i * dir_y,
                    start.z + i * dir_z,
                )
            })
            .collect()
    }

    fn move_down(&mut self) {
        for c in self.cubes.iter_mut() {
            assert!(c.z > 1);
            c.z -= 1;
        }
    }
}

pub fn create_structure(input: &str) -> FxHashMap<usize, Vec<usize>> {
    let mut bricks = parse_input(input);

    const GROUND: usize = usize::MAX;

    // Populate a 3D grid
    let mut grid_state = [[[None; SIZE as usize]; SIZE as usize]; Z_SIZE as usize];
    // populate ground
    for y in 0..SIZE {
        for x in 0..SIZE {
            grid_state[0][y as usize][x as usize] = Some(GROUND);
        }
    }

    // populate bricks in initial state
    for brick in bricks.iter() {
        for c in &brick.cubes {
            assert!(grid_state[c.z as usize][c.y as usize][c.x as usize] == None);
            grid_state[c.z as usize][c.y as usize][c.x as usize] = Some(brick.id);
        }
    }

    // Move all bricks as far down as they can fall before being obstructed
    // TODO: possible optimisation - move each brick all the way to the bottom in one go
    let mut done = false;
    while !done {
        done = true;

        for brick in bricks.iter_mut() {
            // a brick can move if no other brick below it - all cubes will be empty, or part of
            // this brick
            let can_move = brick.cubes.iter().all(|c| {
                let state = grid_state[c.z as usize - 1][c.y as usize][c.x as usize];
                state == None || state == Some(brick.id)
            });
            if can_move {
                // if this brick could move, flag that we need to start the loop again
                done = false;
                // erase the brick, move it, then draw again
                for c in &brick.cubes {
                    grid_state[c.z as usize][c.y as usize][c.x as usize] = None;
                }
                brick.move_down();
                for c in &brick.cubes {
                    grid_state[c.z as usize][c.y as usize][c.x as usize] = Some(brick.id);
                }
            }
        }
    }

    // TODO: possible optimisation - vector instead of hashmap
    // For each brick, find all the bricks it is supported by checking if any of the cubes move down into a brick
    // instead of ground, empty space or itself
    let mut result = FxHashMap::default();
    for brick in bricks {
        let supported_by = brick
            .cubes
            .iter()
            .map(|c| grid_state[c.z as usize - 1][c.y as usize][c.x as usize])
            .filter(|&c| c != None && c != Some(GROUND) && c != Some(brick.id))
            .map(|c| c.unwrap())
            .unique()
            .collect::<Vec<_>>();
        result.insert(brick.id, supported_by);
    }
    result
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .trim()
        .split("\n")
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line
                .split("~")
                .map(|p| Point::from_str(p))
                .collect_tuple()
                .unwrap();
            Brick::new(i, start, end)
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let structure = create_structure(input);

    // Using the structure, find all the bricks that are supported by only one brick - these can't be disintegrated
    // Return the number that can be disintegrated by subtracting from the total
    Some(
        structure.len()
            - structure
                .iter()
                .filter_map(|(_, supported_by)| (supported_by.len() == 1).then_some(supported_by))
                .unique()
                .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let structure = create_structure(input);

    // Use the part 1 solution to find all the bricks that will cause others to fall if disintegrated
    let to_disintegrate = structure
        .iter()
        .filter_map(|(_, supported_by)| (supported_by.len() == 1).then(|| supported_by[0]))
        .unique()
        .collect::<Vec<_>>();

    // Create an inverse index of the structure to find what other bricks each is supporting
    let mut supports_map = FxHashMap::default();
    for (&brick, supported_by) in &structure {
        for s in supported_by {
            supports_map
                .entry(s)
                .and_modify(|v: &mut Vec<usize>| v.push(brick))
                .or_insert(vec![brick]);
        }
    }

    let mut total = 0;
    for d in to_disintegrate {
        // Create a chain reaction by disintegrating this brick, then walking through
        // all the bricks it is supporting, and that they are in turn supporting,
        // marking which bricks get disintegrated
        let mut disintegrated = vec![false; structure.len()];
        let mut queue = VecDeque::new();
        queue.push_back(d);

        while let Some(brick) = queue.pop_front() {
            disintegrated[brick] = true;
            if supports_map.contains_key(&brick) {
                for chained_brick in &supports_map[&brick] {
                    // The brick will be disintegrated if all the bricks supporting it have
                    // been disintegrated
                    if structure[chained_brick].iter().all(|&b| disintegrated[b]) {
                        queue.push_back(*chained_brick);
                    }
                }
            }
        }
        // Exclude the origin from the total
        total += disintegrated.iter().filter(|v| **v).count() - 1;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
