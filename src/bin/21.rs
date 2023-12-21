use std::collections::HashSet;

advent_of_code::solution!(21);

fn count_destinations(input: &str, num_steps: u32) -> Option<u32> {
    let lines = input.trim().split("\n").collect::<Vec<_>>();
    let (w, h) = (lines[0].len() as i32, lines.len() as i32);

    let mut grid = vec![vec![0; w as usize]; h as usize];
    let mut start = None;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => grid[y][x] = 1,
                'S' => start = Some((y as i32, x as i32)),
                _ => panic!("Invalid input"),
            }
        }
    }

    let mut locations = HashSet::new();

    locations.insert(start.unwrap());
    for _ in 0..num_steps {
        let mut new_locations = HashSet::new();
        for (x, y) in locations {
            // Left
            if grid[y.rem_euclid(h) as usize][(x - 1).rem_euclid(w) as usize] == 0 {
                new_locations.insert((x - 1, y));
            }
            // Up
            if grid[(y - 1).rem_euclid(h) as usize][x.rem_euclid(w) as usize] == 0 {
                new_locations.insert((x, y - 1));
            }
            // Right
            if grid[y.rem_euclid(h) as usize][(x + 1).rem_euclid(w) as usize] == 0 {
                new_locations.insert((x + 1, y));
            }
            // Down
            if grid[(y + 1).rem_euclid(h) as usize][x.rem_euclid(w) as usize] == 0 {
                new_locations.insert((x, y + 1));
            }
        }
        locations = new_locations;
        println!(
            "Total nodes {}, Central Nodes {}",
            locations.len(),
            locations
                .iter()
                .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < w && *y < h)
                .count()
        );
    }

    Some(locations.len() as u32)
}

fn _print_locations(locations: &HashSet<(i32, i32)>, grid: &Vec<Vec<i32>>) {
    let (w, h) = (grid[0].len() as i32, grid.len() as i32);
    for y in -h..h * 2 {
        for x in -w..w * 2 {
            if locations.contains(&(x as i32, y as i32)) {
                print!("O");
            } else {
                print!(
                    "{}",
                    match grid[y.rem_euclid(h) as usize][x.rem_euclid(w) as usize] {
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
    count_destinations(input, 26501365)
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
        let result = count_destinations(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
        let result = count_destinations(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(50));
        let result = count_destinations(&advent_of_code::template::read_file("examples", DAY), 50);
        assert_eq!(result, Some(1594));
        let result = count_destinations(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(6536));
        // let result = count_destinations(&advent_of_code::template::read_file("examples", DAY), 500);
        // assert_eq!(result, Some(167004));
        // let result =
        //     count_destinations(&advent_of_code::template::read_file("examples", DAY), 1000);
        // assert_eq!(result, Some(668697));
        // let result =
        //     count_destinations(&advent_of_code::template::read_file("examples", DAY), 5000);
        // assert_eq!(result, Some(16733044));
    }
}
