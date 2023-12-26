use std::collections::HashSet;

advent_of_code::solution!(21);

fn count_destinations(input: &str, num_steps: u32) -> Option<usize> {
    let lines = input.trim().split("\n").collect::<Vec<_>>();
    let size = lines.len();
    assert_eq!(lines[0].len(), size);

    let mut grid = vec![vec![0; size]; size];
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

    const GARDEN_SIZE: usize = 7;

    locations.insert(start.unwrap());
    for _ in 0..num_steps {
        let mut new_locations = HashSet::new();
        let mut done = false;
        for (x, y) in locations {
            // Left
            if grid[y.rem_euclid(size as i32) as usize][(x - 1).rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x - 1, y));
                if x == -((GARDEN_SIZE / 2) as i32 * size as i32) + 1 {
                    done = true;
                }
            }
            // Up
            if grid[(y - 1).rem_euclid(size as i32) as usize][x.rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x, y - 1));
                if y == -((GARDEN_SIZE / 2) as i32 * size as i32) + 1 {
                    done = true;
                }
            }
            // Right
            if grid[y.rem_euclid(size as i32) as usize][(x + 1).rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x + 1, y));
                if x == size as i32 * (GARDEN_SIZE / 2 + 1) as i32 - 1 {
                    done = true;
                }
            }
            // Down
            if grid[(y + 1).rem_euclid(size as i32) as usize][x.rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x, y + 1));
                if y == size as i32 * (GARDEN_SIZE / 2 + 1) as i32 - 1 {
                    done = true;
                }
            }
        }
        locations = new_locations;
        if done {
            let mut garden_count = vec![vec![0; GARDEN_SIZE as usize]; GARDEN_SIZE as usize];
            let offset = size as i32 * (GARDEN_SIZE / 2) as i32;
            for y in 0..size * GARDEN_SIZE {
                for x in 0..size * GARDEN_SIZE {
                    if locations.contains(&((x as i32 - offset), (y as i32 - offset))) {
                        garden_count[y / size][x / size] += 1;
                    }
                }
            }

            // TODO: assumption here of constant being the right one
            let n = num_steps as usize / size;
            let total = n
                * (garden_count[1][1]
                    + garden_count[5][1]
                    + garden_count[1][5]
                    + garden_count[5][5])
                + (n - 1)
                    * (garden_count[1][2]
                        + garden_count[1][4]
                        + garden_count[5][2]
                        + garden_count[5][4])
                + garden_count[0][3]
                + garden_count[3][0]
                + garden_count[3][6]
                + garden_count[6][3]
                + n * n * garden_count[1][3]
                + (n - 1) * (n - 1) * garden_count[2][3];
            return Some(total);
        }
    }

    Some(locations.len())
}

fn _print_locations(locations: &HashSet<(i32, i32)>, grid: &Vec<Vec<i32>>) {
    const GARDEN_SIZE: i32 = 3;

    let (w, h) = (grid[0].len() as i32, grid.len() as i32);
    for y in -h * (GARDEN_SIZE / 2)..h * (GARDEN_SIZE / 2 + 1) {
        for x in -w * (GARDEN_SIZE / 2)..w * (GARDEN_SIZE / 2 + 1) {
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

pub fn part_one(input: &str) -> Option<usize> {
    count_destinations(input, 64)
}

pub fn part_two(input: &str) -> Option<usize> {
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
        // let result = count_destinations(&advent_of_code::template::read_file("examples", DAY), 50);
        // assert_eq!(result, Some(1594));
        // let result = count_destinations(&advent_of_code::template::read_file("examples", DAY), 100);
        // assert_eq!(result, Some(6536));
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
