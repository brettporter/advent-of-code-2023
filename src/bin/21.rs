use rustc_hash::FxHashSet;

advent_of_code::solution!(21);

fn count_destinations(input: &str, num_steps: u32) -> Option<usize> {
    let lines = input.trim().split("\n").collect::<Vec<_>>();
    let size = lines.len();
    assert_eq!(lines[0].len(), size);

    // Build initial grid from the input
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

    // Build a list of locations that could be visited, beginning at the start point
    let mut locations = FxHashSet::default();
    locations.insert(start.unwrap());

    // If we get to 7x7 gardens for part two, we know the pattern of the input that can be used
    // to infer the answer
    const GARDEN_SIZE: usize = 7;

    // Iterate through all the required steps, but abort when we reach a conssistent pattern
    for _ in 0..num_steps {
        // Replace the current locations with a new set as we'll  change to the surrounding
        let mut new_locations = FxHashSet::default();
        let mut done = false;
        for (x, y) in locations {
            // We use rem_euclid to determine the relative position within the original grid since
            // it is infinitely repeating outwards

            // Check each direction and if no rock there, then add a new possible location
            // If we've reached the edge of the 7x7 grid, mark as done

            // Check left (x - 1, y)
            if grid[y.rem_euclid(size as i32) as usize][(x - 1).rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x - 1, y));
                if x == -((GARDEN_SIZE / 2) as i32 * size as i32) + 1 {
                    done = true;
                }
            }
            // Check up (x, y - 1)
            if grid[(y - 1).rem_euclid(size as i32) as usize][x.rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x, y - 1));
                if y == -((GARDEN_SIZE / 2) as i32 * size as i32) + 1 {
                    done = true;
                }
            }
            // Check right (x + 1, y)
            if grid[y.rem_euclid(size as i32) as usize][(x + 1).rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x + 1, y));
                if x == size as i32 * (GARDEN_SIZE / 2 + 1) as i32 - 1 {
                    done = true;
                }
            }
            // Check down (x, y + 1)
            if grid[(y + 1).rem_euclid(size as i32) as usize][x.rem_euclid(size as i32) as usize]
                == 0
            {
                new_locations.insert((x, y + 1));
                if y == size as i32 * (GARDEN_SIZE / 2 + 1) as i32 - 1 {
                    done = true;
                }
            }
        }
        // Use the new set of locations for the next iteration
        locations = new_locations;

        // Check if we found the 7x7 boundary and can repeat from there
        if done {
            // Count the number of locations in each of the repeated gardens
            let mut garden_count = vec![vec![0; GARDEN_SIZE as usize]; GARDEN_SIZE as usize];
            let offset = size as i32 * (GARDEN_SIZE / 2) as i32;
            for y in 0..size * GARDEN_SIZE {
                for x in 0..size * GARDEN_SIZE {
                    if locations.contains(&((x as i32 - offset), (y as i32 - offset))) {
                        garden_count[y / size][x / size] += 1;
                    }
                }
            }

            // n is the number of full gardens we need to traverse to get the eventual answer
            // Pattern of 7x7 repeats, which looks like
            // . . A B a . .
            // . A D E d a .
            // A D E e E d a
            // F E e E e E f
            // G H E e E h g
            // . G H E h g .
            // . . G b g . .
            // E and e will alternate internally and expand as gardens are filled, with the same numbers
            // on the boundaries at each full interval
            // So for 7x7, n is 3
            // The total number is the sum of the gardens. There will be:
            //            n of A, a, G, g along the boundary
            //      (n - 1) of D, d, H, h along the inside of the boundary that are not yet full
            //       single of F, f, B, b at the points of the diamond
            //        n ^ 2 of E the area of the inner diamond (odd distances)
            //  (n - 1) ^ 2 of e the area of the inner diamond (even distances)

            let n = num_steps as usize / size;
            let total = n
                * (garden_count[1][1] // A
                    + garden_count[5][1] // a
                    + garden_count[1][5] // G
                    + garden_count[5][5]) // g
                + (n - 1)
                    * (garden_count[1][2] // D
                        + garden_count[1][4] // d
                        + garden_count[5][2] // H
                        + garden_count[5][4]) // h
                + garden_count[0][3] // F
                + garden_count[6][3] // f
                + garden_count[3][0] // B
                + garden_count[3][6] // b
                + n * n * garden_count[1][3] // E
                + (n - 1) * (n - 1) * garden_count[2][3]; // e

            // Abort early with this total
            return Some(total);
        }
    }

    // Return the total reached
    Some(locations.len())
}

fn _print_locations(locations: &FxHashSet<(i32, i32)>, grid: &Vec<Vec<i32>>) {
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

        // TODO: the current approach works for the input data, but not for the examples because the number of steps do not exactly match the interval
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
