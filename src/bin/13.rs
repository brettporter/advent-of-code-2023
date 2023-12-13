use itertools::Itertools;

advent_of_code::solution!(13);

fn find_mirror(section: Vec<usize>, error_target: usize) -> Option<u32> {
    // The right mirror will be bounded by either the start or the end, find valid boundaries for
    // each of these
    let first = section[0];
    for i in (1..section.len()).step_by(2) {
        if check_lines(first, section[i], error_target) <= error_target {
            // For this valid boundary, check that the range between them is correct
            // and return the number of elements to the left of the mirror
            if let Some(value) = check_range(0, i, &section, error_target) {
                // Found a mirror with correct smudge level
                return Some(value as u32);
            }
        }
    }
    // Repeat for last
    let last = section[section.len() - 1];
    for i in (0..section.len() - 1).rev().step_by(2) {
        if check_lines(last, section[i], error_target) <= error_target {
            if let Some(value) = check_range(i, section.len() - 1, &section, error_target) {
                return Some(value as u32);
            }
        }
    }
    // No mirror found
    None
}

fn check_range(
    start: usize,
    end: usize,
    section: &Vec<usize>,
    error_target: usize,
) -> Option<usize> {
    // Check all rows of start to end (inclusive) to ensure they are mirrored

    let mut errors = 0;
    let middle = (end - (start + 1)) / 2;
    for i in 0..=middle {
        // Capture the number of errors for this pair of lines
        errors += check_lines(section[start + i], section[end - i], error_target);
        // If there are too many errors, this is not a valid mirror
        if errors > error_target {
            return None;
        }
    }
    // Check exactly the right number of smudges
    if errors == error_target {
        // Return the number of lines to the left, which is the midpoint of the range
        Some(middle + start + 1)
    } else {
        None
    }
}

fn check_lines(line1: usize, line2: usize, error_target: usize) -> usize {
    // xor will tell us how many bits are different
    let mut diff = line1 ^ line2;
    let mut errors = 0;
    // Find the count of different bits. Each iteration eliminates the topmost
    // bit
    while diff > 0 {
        diff &= diff - 1;
        errors += 1;
        if errors > error_target {
            // If there are too many errors on this line, fail fast
            break;
        }
    }
    errors
}

fn process(input: &str, error_target: usize) -> u32 {
    let mut v = Vec::new();

    let mut current_section = Vec::new();
    for line in input.split("\n") {
        if line == "" {
            v.push(current_section);
            current_section = Vec::new();
        } else {
            current_section.push(
                line.as_bytes()
                    .iter()
                    .map(|c| if *c == b'#' { 1usize } else { 0usize })
                    .collect_vec(),
            );
        }
    }

    // let p = parser!(sections(
    //     lines(char_of(".#")+)
    // ));
    // let v = p.parse(input).unwrap();

    let mut total = 0;

    for section in v {
        // Create horizontal rows as a bitmap (limits the number of rows to 64)
        // Each row will be represented as a single usize and we can use bitwise
        // operations to check for differences
        let horizontal = section
            .iter()
            .map(|row| {
                let mut value = 0;
                for i in 0..row.len() {
                    value |= row[i] << i;
                }
                value
            })
            .collect_vec();

        if let Some(result) = find_mirror(horizontal, error_target) {
            total += result * 100;
            continue;
        }

        // Create vertical rows as a bitmap (transposed so we can use same find algorithm)
        let vertical = (0..section[0].len())
            .map(|col| {
                let mut value = 0;
                for (i, row) in section.iter().enumerate() {
                    value |= row[col] << i;
                }
                value
            })
            .collect_vec();

        if let Some(result) = find_mirror(vertical, error_target) {
            total += result;
        }
    }
    total as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(process(input, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(process(input, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
