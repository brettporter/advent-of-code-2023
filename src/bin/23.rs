use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::FxHashSet;

advent_of_code::solution!(23);

struct Junction {
    x: usize,
    y: usize,
    from: (usize, usize),
    distance: i32,
}

impl Junction {
    fn new(to: (usize, usize), from: (usize, usize), distance: i32) -> Self {
        let (x, y) = to;
        Self {
            x,
            y,
            from,
            distance,
        }
    }
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

    let visited = FxHashSet::default();

    let mut valid_paths = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((Junction::new(start, start, 0), visited));

    let mut max_distance = 0;

    // Traverse remaining junction points that haven't been visited on this path
    // TODO: alternative could be to just search for junction points and search tree?
    while let Some((junction, visited)) = queue.pop_front() {
        let mut from = junction.from;
        let mut cur = (junction.x, junction.y);
        let mut distance = 0;

        loop {
            distance += 1;
            if cur == dest {
                if distance + junction.distance > max_distance {
                    // TODO: remove debug once it's fast enough not to need to know
                    println!(
                        "Found dest {} - still to process alternatives: {}",
                        distance + junction.distance,
                        queue.len()
                    );
                    max_distance = distance + junction.distance;
                }
                valid_paths.push(distance + junction.distance);
                break;
            }

            let (x, y) = cur;

            let mut choices = Vec::new();
            // left
            if x > 0 {
                let v = grid[y][x - 1];
                if (!slippery && v != '#') || v == '.' || v == '<' {
                    if from != (x - 1, y) {
                        choices.push((x - 1, y));
                    }
                }
            }
            // right
            if x < w - 1 {
                let v = grid[y][x + 1];
                if (!slippery && v != '#') || v == '.' || v == '>' {
                    if from != (x + 1, y) {
                        choices.push((x + 1, y));
                    }
                }
            }
            // up
            if y > 0 {
                let v = grid[y - 1][x];
                if (!slippery && v != '#') || v == '.' || v == '^' {
                    if from != (x, y - 1) {
                        choices.push((x, y - 1));
                    }
                }
            }
            // down
            if y < h - 1 {
                let v = grid[y + 1][x];
                if (!slippery && v != '#') || v == '.' || v == 'v' {
                    if from != (x, y + 1) {
                        choices.push((x, y + 1));
                    }
                }
            }

            // Dead end - stop looking for a junction on this route
            if choices.len() == 0 {
                break;
            }

            // Only one choice - continue path as not at a junction
            if choices.len() == 1 {
                from = (x, y);
                cur = *choices.first().unwrap();
                if visited.contains(&cur) {
                    break;
                }
            } else {
                // Reached a junction point - add the choices to the queue of paths to explore
                for n in choices {
                    // Take a copy of the visited set including the junction for each path
                    let mut visited = visited.clone();
                    visited.insert(cur);
                    queue.push_back((Junction::new(n, cur, junction.distance + distance), visited));
                }
                // Hand back to the main loop to process these options
                break;
            }
        }
    }

    Some(valid_paths.into_iter().max().unwrap() - 1)
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
