use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(23);

// TODO: could we reuse from Day 17?
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Node {
    loc: (usize, usize),
    incoming_direction: Direction,
}

#[derive(Debug)]
struct Edge {
    from: Node,
    to: Node,
    distance: i32,
}

fn traverse_grid(input: &str, slippery: bool) -> Option<i32> {
    let grid = input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let (w, h) = (grid[0].len(), grid.len());
    let start = Node {
        loc: (1, 0),
        incoming_direction: Direction::Down,
    };
    let dest = Node {
        loc: (w - 2, h - 1),
        incoming_direction: Direction::Down,
    };

    // TODO: too much copying of nodes instead of references?
    let mut queue = VecDeque::new();
    queue.push_back((start, Direction::Down));

    // TODO: ignore direction if slippery, as it is inherently directed

    let mut nodes = FxHashSet::default();
    nodes.insert(start);
    nodes.insert(dest);

    let mut edges = Vec::new(); // TODO: correct data structure?

    // TODO: is it faster to pre-identify all the potential nodes based on graph and stop edges at all of these, even if fewer options in part 1?
    // Traverse remaining junction points that haven't been discovered yet
    while let Some((junction, next_direction)) = queue.pop_front() {
        // move in the desired direction of this path
        let mut loc = next_direction.translate(junction.loc);
        let mut incoming_direction = next_direction;

        let mut distance = 0;

        loop {
            distance += 1;

            if loc == dest.loc {
                edges.push(Edge {
                    from: junction,
                    to: Node {
                        loc,
                        incoming_direction,
                    },
                    distance,
                });
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
                // if this node exists coming from the given incoming direction, add an edge and stop, otherwise add a node and push back all the points
                let new_node = Node {
                    loc,
                    incoming_direction,
                };

                if !nodes.contains(&new_node) {
                    nodes.insert(new_node);
                    for d in choices {
                        queue.push_back((new_node, d));
                    }
                }
                edges.push(Edge {
                    from: junction,
                    to: new_node,
                    distance,
                });

                // Hand back to the main loop to process these options
                break;
            }
        }
    }

    // Topological sort
    let mut stack = VecDeque::new(); // TODO: return from dfs?
    let mut visited = FxHashSet::default(); // TODO: size of nodes
    for n in nodes {
        topo_sort(n, &edges, &mut visited, &mut stack);
    }

    println!("Topological sort: {:#?}", stack);

    // TODO: is there a way to eliminate cycles in the graph or filter out here?
    // Find longest path
    let mut max_distance = FxHashMap::default();
    for n in stack {
        // TODO: better data structure to find incoming neighbours
        let d = edges
            .iter()
            .filter(|e| e.to == n)
            .map(|e| max_distance.get(&e.from).unwrap_or(&0) + e.distance)
            .max();
        max_distance.insert(n, d.unwrap_or(0));
    }

    Some(*max_distance.get(&dest).unwrap())
}

fn topo_sort(
    n: Node,
    edges: &Vec<Edge>,
    visited: &mut FxHashSet<Node>,
    stack: &mut VecDeque<Node>,
) {
    if !visited.contains(&n) {
        visited.insert(n);
        // TODO: more efficient data structure?
        for e in edges.iter().filter(|e| e.to == n) {
            topo_sort(e.from, edges, visited, stack);
        }
        stack.push_back(n);
    }
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
