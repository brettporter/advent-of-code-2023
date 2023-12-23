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

pub fn part_two(input: &str) -> Option<i32> {
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
    queue.push_back((start, start, 0, visited));

    let mut max_Distance = 0;

    // TODO: alternative could be to just search for junction points and search tree
    while let Some(((x, y), from, total_distance, visited)) = queue.pop_front() {
        // println!("Following path from junction {} {}", x, y);
        if (x, y) == dest {
            if total_distance - 1 > max_Distance {
                println!(
                    "Found dest {} - remaining {}",
                    total_distance - 1,
                    queue.len()
                );
                max_Distance = total_distance - 1;
            }
            valid_paths.push(total_distance - 1);
            continue;
        }

        let mut from = from;
        let mut cur = (x, y);
        let mut distance = 0;

        loop {
            distance += 1;
            let (x, y) = cur;
            if (x, y) == dest {
                if distance + total_distance - 1 > max_Distance {
                    println!(
                        "Found dest {} - remaining {}",
                        distance + total_distance - 1,
                        queue.len()
                    );
                    max_Distance = distance + total_distance - 1;
                }
                valid_paths.push(distance + total_distance - 1);
                break;
            }

            let mut choices = Vec::new();
            // left
            if x > 0 {
                if grid[y][x - 1] != '#' {
                    if from != (x - 1, y) {
                        choices.push((x - 1, y));
                    }
                }
            }
            // right
            if x < w - 1 {
                if grid[y][x + 1] != '#' {
                    if from != (x + 1, y) {
                        choices.push((x + 1, y));
                    }
                }
            }
            // up
            if y > 0 {
                if grid[y - 1][x] != '#' {
                    if from != (x, y - 1) {
                        choices.push((x, y - 1));
                    }
                }
            }
            // down
            if y < h - 1 {
                if grid[y + 1][x] != '#' {
                    if from != (x, y + 1) {
                        choices.push((x, y + 1));
                    }
                }
            }

            if choices.len() == 0 {
                break;
            }
            if choices.len() == 1 {
                from = (x, y);
                cur = *choices.first().unwrap();
                continue;
            }

            // Junction point
            // println!("Found junction {} {} -> {:?}", x, y, choices);
            if visited.contains(&(x, y)) {
                break;
            }

            let mut visited = visited.clone();
            visited.insert((x, y));
            for n in choices {
                queue.push_back((n, (x, y), total_distance + distance, visited.clone()));
            }
            break;
        }
    }

    Some(*valid_paths.iter().max().unwrap() as i32)
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
