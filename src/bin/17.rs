use aoc_parse::{parser, prelude::*};
use priority_queue::DoublePriorityQueue;

advent_of_code::solution!(17);

fn build_weighted_grid(input: &str) -> Vec<Vec<usize>> {
    let p = parser!(lines(digit+));

    p.parse(input).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = build_weighted_grid(input.trim());

    let mut queue = DoublePriorityQueue::new();

    let w = grid[0].len();
    let h = grid.len();

    let start = (0, 0);
    // let dest = (w - 1, h - 1);
    let dest = (8, 1);
    queue.push(start, 0);

    let mut cost_tally = vec![vec![usize::MAX; w]; h];
    let mut came_from = vec![vec![None; w]; h];
    cost_tally[0][0] = 0;

    fn find_neighbours(current: (usize, usize), w: usize, h: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if current.0 > 0 {
            result.push((current.0 - 1, current.1));
        }
        if current.0 < w - 1 {
            result.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            result.push((current.0, current.1 - 1));
        }
        if current.1 < h - 1 {
            result.push((current.0, current.1 + 1));
        }
        result
    }

    while let Some((current, weight)) = queue.pop_min() {
        if current == dest {
            println!(
                "Breaking current {} {} is dest {} {} which came from {:?}",
                current.0, current.1, dest.0, dest.1, came_from[current.1][current.0]
            );
            break;
        }
        for n in find_neighbours(current, w, h) {
            let cost = cost_tally[current.1][current.0] + grid[n.1][n.0];
            println!(
                "Compare cost: {} {} -> {} {}; cost = {} was {}",
                current.0, current.1, n.0, n.1, cost, cost_tally[n.1][n.0]
            );
            if cost < cost_tally[n.1][n.0] {
                println!("Improved cost");
                // test we haven't done 3 consecutive moves in the same direction
                let mut prev = current;
                for _ in 0..3 {
                    if let Some(p) = came_from[prev.1][prev.0] {
                        prev = p;
                    }
                }

                // test horizontal movement
                if n.1 == prev.1 && n.0.abs_diff(prev.0) > 3 {
                    println!("Skipping h {} {} -> {} {}", prev.0, prev.1, n.0, n.1);
                    continue;
                }
                // test vertical movement
                if n.0 == prev.0 && n.1.abs_diff(prev.1) > 3 {
                    println!("Skipping v {} {} -> {} {}", prev.0, prev.1, n.0, n.1);
                    continue;
                }

                cost_tally[n.1][n.0] = cost;
                if let Some(y) = queue.get(&n) {
                    println!("CONTAINS {:?}", y);
                }
                let x = queue.push(n, cost);
                if x.is_some() {
                    println!("REPLACED {} with {}", x.unwrap(), cost);
                }
                println!("n {} {} came from {} {}", n.0, n.1, current.0, current.1);
                came_from[n.1][n.0] = Some(current);
            }
        }
    }

    let mut heat_loss = 0;
    let mut current = dest;

    let mut print_path = vec![vec![' '; w]; h];
    let mut count = -1;
    while current != start {
        println!("{} {}", current.0, current.1);
        count = (count + 1) % 9;
        // print_path[current.1][current.0] = count + 1;
        print_path[current.1][current.0] = '.';

        heat_loss += grid[current.1][current.0];
        current = came_from[current.1][current.0].unwrap();
    }

    for row in print_path {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();

    Some(heat_loss as u32)
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
        // assert_eq!(result, Some(102));
        assert_eq!(result, Some(32));
        todo!();
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
