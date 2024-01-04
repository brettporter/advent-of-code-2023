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

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Node {
    loc: (usize, usize),
    visited: Vec<(usize, usize)>,
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
        visited: vec![],
    };
    let dest = (w - 2, h - 1);

    // TODO: too much copying of nodes instead of references?
    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), Direction::Down));

    // TODO: ignore direction if slippery, as it is inherently directed

    let mut nodes = FxHashSet::default();
    nodes.insert(start);

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

            if loc == dest {
                let mut visited = junction.visited.clone();
                visited.push(dest);
                let nn = Node { loc, visited };
                nodes.insert(nn.clone());
                edges.push(Edge {
                    from: junction,
                    to: nn,
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
                if junction.visited.contains(&loc) {
                    break;
                }

                // TOOD: cleanup unecessary cloning

                let mut visited = junction.visited.clone();
                visited.push(loc);
                let new_node = Node { loc, visited };

                if !nodes.contains(&new_node) {
                    for d in choices {
                        queue.push_back((new_node.clone(), d));
                    }
                    nodes.insert(new_node.clone());
                }
                edges.push(Edge {
                    from: junction,
                    to: new_node.clone(),
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

    // println!("Topological sort: {:#?}", stack);

    // TODO: is there a way to eliminate cycles in the graph or filter out here?
    // TODO: only add the biggest ones...
    // Find longest path
    let mut max_distance = FxHashMap::default();
    for n in stack {
        // TODO: better data structure to find incoming neighbours
        let d = edges
            .iter()
            .filter(|e| e.to == n)
            .map(|e| max_distance.get(&e.from).unwrap_or(&0) + e.distance)
            .max();
        max_distance.insert(n.clone(), d.unwrap_or(0));
    }

    max_distance
        .into_iter()
        .filter_map(|(n, d)| (n.loc == dest).then_some(d))
        .max()
}

fn topo_sort(
    n: Node,
    edges: &Vec<Edge>,
    visited: &mut FxHashSet<Node>,
    stack: &mut VecDeque<Node>,
) {
    if !visited.contains(&n) {
        visited.insert(n.clone());
        // TODO: more efficient data structure?
        for e in edges.iter().filter(|e| e.to == n) {
            topo_sort(e.from.clone(), edges, visited, stack);
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
