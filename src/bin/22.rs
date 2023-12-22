use std::collections::HashSet;

use itertools::Itertools;

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
    start: Point,
    end: Point,
}

impl Brick {
    fn new(id: usize, start: Point, end: Point) -> Self {
        Self { id, start, end }
    }

    fn cubes(&self) -> Vec<Point> {
        let (dx, dy, dz) = self.end.diff(&self.start);
        let brick_len = (dx + dy + dz).abs() + 1;
        let (dir_x, dir_y, dir_z) = (dx.signum(), dy.signum(), dz.signum());

        (0..brick_len)
            .into_iter()
            .map(|i| {
                Point::new(
                    self.start.x + i * dir_x,
                    self.start.y + i * dir_y,
                    self.start.z + i * dir_z,
                )
            })
            .collect()
    }

    fn move_down(&mut self) {
        assert!(self.start.z > 1 && self.end.z > 1);
        self.start.z -= 1;
        self.end.z -= 1;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut bricks = input
        .trim()
        .split("\n")
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line
                .split("~")
                .map(|p| Point::from_str(p))
                .collect_tuple()
                .unwrap();
            Brick::new(i + 1, start, end)
        })
        .collect_vec();

    const EMPTY: usize = 0;
    const GROUND: usize = usize::MAX;

    // TODO: check intersection with other bricks rather than grid state double handling?
    let mut grid_state = [[[EMPTY; SIZE as usize]; SIZE as usize]; Z_SIZE as usize];
    // ground
    for y in 0..SIZE {
        for x in 0..SIZE {
            grid_state[0][y as usize][x as usize] = GROUND;
        }
    }

    for brick in bricks.iter() {
        for c in brick.cubes() {
            assert!(grid_state[c.z as usize][c.y as usize][c.x as usize] == EMPTY);
            grid_state[c.z as usize][c.y as usize][c.x as usize] = brick.id;
        }
    }

    let mut done = false;
    while !done {
        done = true;

        for brick in bricks.iter_mut() {
            let can_move = brick.cubes().iter().all(|c| {
                grid_state[c.z as usize - 1][c.y as usize][c.x as usize] == EMPTY
                    || grid_state[c.z as usize - 1][c.y as usize][c.x as usize] == brick.id
            });
            if can_move {
                done = false;
                // TODO: could be more efficient, currently erase, move, draw
                for c in brick.cubes() {
                    grid_state[c.z as usize][c.y as usize][c.x as usize] = EMPTY;
                }
                brick.move_down();
                for c in brick.cubes() {
                    grid_state[c.z as usize][c.y as usize][c.x as usize] = brick.id;
                }
            }
        }
    }

    let mut cannot_disintegrate = HashSet::new();
    for brick in bricks.iter() {
        let supported_by = brick
            .cubes()
            .iter()
            .map(|c| grid_state[c.z as usize - 1][c.y as usize][c.x as usize])
            .filter(|&c| c != EMPTY && c != GROUND && c != brick.id)
            .unique()
            .collect::<Vec<_>>();

        if supported_by.len() == 1 {
            cannot_disintegrate.extend(supported_by);
        }
    }

    Some(bricks.len() - cannot_disintegrate.len())
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
