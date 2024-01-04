use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(23);

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn translate(&self, junction: (usize, usize)) -> (usize, usize) {
        let (x, y) = junction;
        match self {
            Self::Up => (x, y - 1),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
            Self::Right => (x + 1, y),
        }
    }
}

#[derive(PartialEq, Clone)]
struct Edge {
    from_idx: usize,
    distance: i32,
}

fn traverse_grid(input: &str, slippery: bool) -> Option<i32> {
    let grid = input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let (w, h) = (grid[0].len(), grid.len());
    let start = (1, 0);
    let dest = (w - 2, h - 1);

    let mut edges: Vec<Vec<Edge>> = Vec::new();
    let mut nodes = Vec::new();
    let start_idx = nodes.len();
    nodes.push(start);
    let dest_idx = nodes.len();
    nodes.push(dest);

    let mut queue = VecDeque::new();
    queue.push_back((start_idx, Direction::Down));

    // Traverse remaining junction points that haven't been discovered yet
    while let Some((junction, next_direction)) = queue.pop_front() {
        // move in the desired direction of this path
        let mut loc = next_direction.translate(nodes[junction]);
        let mut incoming_direction = next_direction;

        let mut distance = 0;

        loop {
            distance += 1;

            if loc == dest {
                let edge = Edge {
                    from_idx: junction,
                    distance,
                };
                if !edges[dest_idx].contains(&edge) {
                    edges[dest_idx].push(edge);
                }
                break;
            }

            let (x, y) = loc;

            let mut choices = Vec::new();
            // left
            if incoming_direction != Direction::Right {
                if x > 0 {
                    let v = grid[y][x - 1];
                    if (!slippery && v != '#') || v == '.' || v == '<' {
                        choices.push(Direction::Left);
                    }
                }
            }
            // right
            if incoming_direction != Direction::Left {
                if x < w - 1 {
                    let v = grid[y][x + 1];
                    if (!slippery && v != '#') || v == '.' || v == '>' {
                        choices.push(Direction::Right);
                    }
                }
            }
            // up
            if incoming_direction != Direction::Down {
                if y > 0 {
                    let v = grid[y - 1][x];
                    if (!slippery && v != '#') || v == '.' || v == '^' {
                        choices.push(Direction::Up);
                    }
                }
            }
            // down
            if incoming_direction != Direction::Up {
                if y < h - 1 {
                    let v = grid[y + 1][x];
                    if (!slippery && v != '#') || v == '.' || v == 'v' {
                        choices.push(Direction::Down);
                    }
                }
            }

            // Dead end - stop looking for a junction on this route
            if choices.len() == 0 {
                break;
            }

            // Only one choice - continue path as not at a junction
            if choices.len() == 1 {
                incoming_direction = *choices.first().unwrap();
                loc = incoming_direction.translate(loc);
            } else {
                // Reached a junction point
                // if this edge has not been found yet, add a new node if needed, then an edge, and explore any outward junction points
                let loc_idx = if let Some(idx) = nodes.iter().position(|&n| n == loc) {
                    idx
                } else {
                    nodes.push(loc);
                    edges.resize(nodes.len(), vec![]);
                    nodes.len() - 1
                };
                let edge = Edge {
                    from_idx: junction,
                    distance,
                };

                if !edges[loc_idx].contains(&edge) {
                    for d in choices {
                        queue.push_back((loc_idx, d));
                    }
                    edges[loc_idx].push(edge);
                }

                // Hand back to the main loop to process these options
                break;
            }
        }
    }

    let mut visited = vec![false; nodes.len()];
    find_max_distance(dest_idx, start_idx, &edges, &mut visited)
}

fn find_max_distance(
    n: usize,
    start: usize,
    edges: &Vec<Vec<Edge>>,
    visited: &mut Vec<bool>,
) -> Option<i32> {
    if n == start {
        return Some(0);
    }

    let mut max_distance = None;
    if !visited[n] {
        visited[n] = true;
        for e in &edges[n] {
            if let Some(d) = find_max_distance(e.from_idx, start, edges, visited) {
                max_distance = max_distance.max(Some(d + e.distance));
            }
        }
        visited[n] = false;
    }
    max_distance
}

pub fn part_one(input: &str) -> Option<i32> {
    traverse_grid(input, true)
}

pub fn part_two(input: &str) -> Option<i32> {
    traverse_grid(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
