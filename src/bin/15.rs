advent_of_code::solution!(15);

fn hash_string(s: &str) -> u8 {
    let mut count = 0u32;
    for c in s.as_bytes() {
        count += *c as u32;
        count *= 17;
        count %= 256;
    }
    count as u8
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(",").map(|s| hash_string(s) as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_algorithm() {
        assert_eq!(hash_string("HASH"), 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
