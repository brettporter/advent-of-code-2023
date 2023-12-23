use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(21);

fn count_destinations(input: &str, num_steps: u32) -> Option<u32> {
    let lines = input.trim().split("\n").collect::<Vec<_>>();
    let size = lines.len();
    assert_eq!(lines[0].len(), size);

    let mut grid = vec![vec![0; size]; size];
    let mut start = None;

    let mut num_rocks = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    num_rocks += 1;
                    grid[y][x] = 1
                }
                'S' => start = Some((y as i32, x as i32)),
                _ => panic!("Invalid input"),
            }
        }
    }

    println!("Rocks = {num_rocks}");

    let mut locations = HashSet::new();
    let mut locations_len = 1;

    let mut full_grid = 0;
    // let mut next_cube = 2;

    locations.insert(start.unwrap());
    for i in 0..num_steps {
        let mut new_locations = HashSet::new();
        let mut done = false;
        for (x, y) in locations {
            // Left
            if grid[y.rem_euclid(size as i32) as usize][(x - 1).rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x - 1, y));
                if x == -((GARDEN_SIZE / 2) * size as i32) + 1 {
                    done = true;
                }
            }
            // Up
            if grid[(y - 1).rem_euclid(size as i32) as usize][x.rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x, y - 1));
                if y == -((GARDEN_SIZE / 2) * size as i32) + 1 {
                    done = true;
                }
            }
            // Right
            if grid[y.rem_euclid(size as i32) as usize][(x + 1).rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x + 1, y));
                if x == size as i32 * (GARDEN_SIZE / 2 + 1) - 1 {
                    done = true;
                }
            }
            // Down
            if grid[(y + 1).rem_euclid(size as i32) as usize][x.rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x, y + 1));
                if y == size as i32 * (GARDEN_SIZE / 2 + 1) - 1 {
                    done = true;
                }
            }
        }
        println!(
            "Step {}: Total nodes {}, Added Nodes {}",
            i + 1,
            new_locations.len(),
            new_locations.len() - locations_len,
        );
        locations = new_locations;
        locations_len = locations.len();
        if done {
            break;
        }
        if locations.contains(&(-(size as i32 * full_grid), -(size as i32 * full_grid)))
            && locations.contains(&(
                -(size as i32 * full_grid),
                (size as i32 * (full_grid + 1)) - 1,
            ))
            && locations.contains(&(
                (size as i32 * (full_grid + 1)) - 1,
                -(size as i32 * full_grid),
            ))
            && locations.contains(&(
                (size as i32 * (full_grid + 1)) - 1,
                (size as i32 * (full_grid + 1)) - 1,
            ))
        {
            println!(
                "Total nodes {}, Central Nodes {}",
                locations.len(),
                locations
                    .iter()
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < size as i32 && *y < size as i32)
                    .count()
            );
            println!(
                "Found {} count {} step {}/{}",
                full_grid,
                locations.len(),
                i,
                num_steps
            );
            let mmx = locations.iter().minmax_by_key(|k| k.0);
            let mmy = locations.iter().minmax_by_key(|k| k.1);
            println!("Dim {:?} {:?}", mmx, mmy);
            full_grid += 1;
        }

        // if locations.len() >= (next_cube * next_cube) * ((next_cube + 1) * (next_cube + 1)) / 4 {
        //     // TODO: check size?
        //     println!(
        //         "Found cube {} step {} count {}",
        //         next_cube,
        //         i,
        //         locations.len()
        //     );
        //     next_cube += 1;
        // }
    }
    _print_locations(&locations, &grid);
    println!();

    Some(locations.len() as u32)
}

const GARDEN_SIZE: i32 = 7;

fn _print_locations(locations: &HashSet<(i32, i32)>, grid: &Vec<Vec<i32>>) {
    let (w, h) = (grid[0].len() as i32, grid.len() as i32);
    let mut garden_count = vec![vec![0; GARDEN_SIZE as usize]; GARDEN_SIZE as usize];
    for y in -h * (GARDEN_SIZE / 2)..h * (GARDEN_SIZE / 2 + 1) {
        for x in -w * (GARDEN_SIZE / 2)..w * (GARDEN_SIZE / 2 + 1) {
            if locations.contains(&(x as i32, y as i32)) {
                garden_count[((y + h * (GARDEN_SIZE / 2)) / h) as usize]
                    [((x + w * (GARDEN_SIZE / 2)) / w) as usize] += 1;
                // print!("O");
            } else {
                // print!(
                //     "{}",
                //     match grid[y.rem_euclid(h) as usize][x.rem_euclid(w) as usize] {
                //         0 => ".",
                //         1 => "#",
                //         _ => panic!(),
                //     }
                // );
            }
        }
        println!();
    }
    println!();

    for (y, row) in garden_count.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 0 {
                print!("    ");
            } else {
                print!("{:6}", col);
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
