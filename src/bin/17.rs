use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use priority_queue::DoublePriorityQueue;

advent_of_code::solution!(17);

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::NONE => Direction::NONE,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}
impl Position {
    fn get_next(
        &self,
        direction: Direction,
        min_span: i32,
        max_span: i32,
        grid: &Vec<Vec<usize>>,
    ) -> Vec<(Position, usize)> {
        let mut result = vec![];
        let mut cost = 0;

        // Find all nodes in the specified direction that are within the span range and within
        // the grid boundaries, calculating the cumulative cost of reaching each of those nodes
        for i in 1..=max_span {
            let new_pos = match direction {
                Direction::UP => Position {
                    x: self.x,
                    y: self.y - i,
                    direction,
                },
                Direction::DOWN => Position {
                    x: self.x,
                    y: self.y + i,
                    direction,
                },
                Direction::LEFT => Position {
                    x: self.x - i,
                    y: self.y,
                    direction,
                },
                Direction::RIGHT => Position {
                    x: self.x + i,
                    y: self.y,
                    direction,
                },
                Direction::NONE => panic!("Invalid direction for next"),
            };

            // At a boundary, stop looking
            if new_pos.x < 0
                || new_pos.x >= grid[0].len() as i32
                || new_pos.y < 0
                || new_pos.y >= grid.len() as i32
            {
                break;
            }

            // Add to the cumulative cost and then add this node if it is in the range
            cost += grid[new_pos.y as usize][new_pos.x as usize];
            if i >= min_span {
                result.push((new_pos, cost));
            }
        }
        result
    }
}

fn build_weighted_grid(input: &str) -> Vec<Vec<usize>> {
    let p = parser!(lines(digit+));

    p.parse(input).unwrap()
}

pub fn calculate_heat_loss(input: &str, min_span: i32, max_span: i32) -> Option<u32> {
    let grid = build_weighted_grid(input.trim());

    let start = Position {
        x: 0,
        y: 0,
        direction: Direction::NONE,
    };
    let dest = (grid[0].len() as i32 - 1, grid.len() as i32 - 1);

    // Use Dijkstra's algorithm with a priority queue
    let mut queue = DoublePriorityQueue::new();
    queue.push(start, 0);

    let mut cost_tally = HashMap::new();
    cost_tally.insert(start, 0);

    while let Some((current, cost)) = queue.pop_min() {
        if (current.x, current.y) == dest {
            return Some(cost as u32);
        }

        for direction in [
            Direction::UP,
            Direction::LEFT,
            Direction::DOWN,
            Direction::RIGHT,
        ] {
            if direction == current.direction || direction.opposite() == current.direction {
                // if it's the same direction skip, as we've already created nodes for all valid spaces forward
                // if it's the opposite direction skip, as we can't go backwards
                continue;
            }

            // Create nodes for all the possible nodes between min_span and max_span and their cumulative costs
            for (n, inc_cost) in current.get_next(direction, min_span, max_span, &grid) {
                let cost = inc_cost + cost;

                // If the cost is lower than previously found (or has not been found), replace it with this node
                // and add it to the priority queue
                if cost < *cost_tally.get(&n).unwrap_or(&usize::MAX) {
                    cost_tally.insert(n, cost);
                    queue.push(n, cost);
                }
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    calculate_heat_loss(input, 1, 3)
}

pub fn part_two(input: &str) -> Option<u32> {
    calculate_heat_loss(input, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
