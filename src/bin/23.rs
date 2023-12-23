use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::FxHashSet;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
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
    queue.push_back((start, visited));

    // TODO: alternative could be to just search for junction points and search tree
    while let Some(((x, y), visited)) = queue.pop_front() {
        let mut visited = visited.clone();
        visited.insert((x, y));
        if (x, y) == dest {
            valid_paths.push(visited);
            continue;
        }

        // left
        if x > 0 {
            if grid[y][x - 1] == '.' || grid[y][x - 1] == '<' {
                if !visited.contains(&(x - 1, y)) {
                    queue.push_back(((x - 1, y), visited.clone()));
                }
            }
        }
        // right
        if x < w - 1 {
            if grid[y][x + 1] == '.' || grid[y][x + 1] == '>' {
                if !visited.contains(&(x + 1, y)) {
                    queue.push_back(((x + 1, y), visited.clone()));
                }
            }
        }
        // up
        if y > 0 {
            if grid[y - 1][x] == '.' || grid[y - 1][x] == '^' {
                if !visited.contains(&(x, y - 1)) {
                    queue.push_back(((x, y - 1), visited.clone()));
                }
            }
        }
        // down
        if y < h - 1 {
            if grid[y + 1][x] == '.' || grid[y + 1][x] == 'v' {
                if !visited.contains(&(x, y + 1)) {
                    queue.push_back(((x, y + 1), visited.clone()));
                }
            }
        }
    }

    valid_paths.iter().map(|p| p.len() - 1).max()
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
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
