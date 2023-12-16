use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(16);

#[derive(Debug)]
struct Beam {
    x: i32,
    y: i32,
    direction: i32, // TODO enum
    done: bool,
}

impl Beam {
    fn move_beam(&mut self, w: i32, h: i32) {
        match self.direction {
            UP => self.y -= 1,
            RIGHT => self.x += 1,
            DOWN => self.y += 1,
            LEFT => self.x -= 1,
            _ => panic!("Invalid direction {}", self.direction),
        }
        if self.x < 0 || self.y < 0 || self.x >= w || self.y >= h {
            self.done = true;
        }
    }
}

const UP: i32 = 0;
const RIGHT: i32 = 1;
const LEFT: i32 = 2;
const DOWN: i32 = 3;

const EMPTY: usize = 0;
const V_SPLIT: usize = 1;
const H_SPLIT: usize = 2;
const MIRROR_BACK: usize = 3;
const MIRROR_FWD: usize = 4;

fn parse_grid(input: &str) -> Vec<Vec<usize>> {
    let p = parser!(lines(
        char_of(".|-\\/")+
    ));
    p.parse(input).unwrap()
}

fn count_energised_tiles(grid: &Vec<Vec<usize>>, beam: Beam) -> u32 {
    let mut light_grid = [[0; 120]; 120];

    let (w, h) = (grid[0].len() as i32, grid.len() as i32);
    let mut beams = vec![beam];

    while let Some(mut beam) = beams.pop() {
        loop {
            if beam.done {
                break;
            }

            let mask = 1 << beam.direction;
            if light_grid[beam.y as usize][beam.x as usize] & mask != 0 {
                break;
            }

            light_grid[beam.y as usize][beam.x as usize] |= mask;

            match grid[beam.y as usize][beam.x as usize] {
                EMPTY => {
                    // move
                }
                V_SPLIT => {
                    match beam.direction {
                        UP | DOWN => {
                            // treat as empty
                        }
                        LEFT | RIGHT => {
                            beam.direction = UP;
                            let mut new_beam = Beam {
                                direction: DOWN,
                                ..beam
                            };
                            new_beam.move_beam(w, h);
                            beams.push(new_beam);
                        }
                        _ => panic!("invalid direction"),
                    }
                }
                H_SPLIT => {
                    match beam.direction {
                        LEFT | RIGHT => {
                            // treat as empty
                        }
                        UP | DOWN => {
                            beam.direction = LEFT;
                            let mut new_beam = Beam {
                                direction: RIGHT,
                                ..beam
                            };
                            new_beam.move_beam(w, h);
                            beams.push(new_beam);
                        }
                        _ => panic!("invalid direction"),
                    }
                }
                MIRROR_BACK => {
                    beam.direction = match beam.direction {
                        RIGHT => DOWN,
                        DOWN => RIGHT,
                        LEFT => UP,
                        UP => LEFT,
                        _ => panic!("invalid direction"),
                    }
                }
                MIRROR_FWD => {
                    beam.direction = match beam.direction {
                        RIGHT => UP,
                        DOWN => LEFT,
                        LEFT => DOWN,
                        UP => RIGHT,
                        _ => panic!("invalid direction"),
                    }
                }
                _ => panic!(
                    "Invalid character {}",
                    grid[beam.y as usize][beam.x as usize]
                ),
            };
            beam.move_beam(w, h);
        }
    }

    light_grid
        .iter()
        .map(|row| row.iter().filter(|c| **c > 0).count())
        .sum::<usize>() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    let beam = Beam {
        x: 0,
        y: 0,
        direction: RIGHT,
        done: false,
    };

    Some(count_energised_tiles(&grid, beam))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    let mut max = 0;

    for y in 0..h {
        // left edge
        let beam = Beam {
            x: 0,
            y,
            direction: RIGHT,
            done: false,
        };

        max = max.max(count_energised_tiles(&grid, beam));

        // right edge
        let beam = Beam {
            x: w - 1,
            y,
            direction: LEFT,
            done: false,
        };

        max = max.max(count_energised_tiles(&grid, beam));
    }

    for x in 0..w {
        // top edge
        let beam = Beam {
            x,
            y: 0,
            direction: DOWN,
            done: false,
        };

        max = max.max(count_energised_tiles(&grid, beam));

        // right edge
        let beam = Beam {
            x,
            y: h - 1,
            direction: UP,
            done: false,
        };

        max = max.max(count_energised_tiles(&grid, beam));
    }
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
