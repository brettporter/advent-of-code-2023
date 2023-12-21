use std::collections::HashSet;

advent_of_code::solution!(21);

fn count_destinations(input: &str, num_steps: u32) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<_>>();
    let (w, h) = (lines[0].len(), lines.len());

    let mut grid = vec![vec![0; w]; h];
    let mut start = None;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => grid[y][x] = 1,
                'S' => start = Some((y, x)),
                _ => panic!("Invalid input"),
            }
        }
    }

    let mut locations = HashSet::new();
    print_locations(&locations, &grid);

    locations.insert(start.unwrap());
    for _ in 0..num_steps {
        let mut new_locations = HashSet::new();
        for (x, y) in locations {
            // Left
            if x > 0 && grid[y][x - 1] == 0 {
                new_locations.insert((x - 1, y));
            }
            // Up
            if y > 0 && grid[y - 1][x] == 0 {
                new_locations.insert((x, y - 1));
            }
            // Right
            if x < w - 1 && grid[y][x + 1] == 0 {
                new_locations.insert((x + 1, y));
            }
            // Down
            if y < h - 1 && grid[y + 1][x] == 0 {
                new_locations.insert((x, y + 1));
            }
        }
        locations = new_locations;
        print_locations(&locations, &grid);
    }

    Some(locations.len() as u32)
}

fn print_locations(locations: &HashSet<(usize, usize)>, grid: &Vec<Vec<i32>>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if locations.contains(&(x, y)) {
                print!("O");
            } else {
                print!(
                    "{}",
                    match grid[y][x] {
                        0 => ".",
                        1 => "#",
                        _ => panic!(),
                    }
                );
            }
        }
        println!();
    }
    println!();
}

pub fn part_one(input: &str) -> Option<u32> {
    count_destinations(input, 64)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = count_destinations(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
