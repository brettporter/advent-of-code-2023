use linked_hash_map::LinkedHashMap;

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
    let mut boxes = vec![LinkedHashMap::new(); 256];

    for s in input.trim().split(',') {
        let v: Vec<&str> = s.split(&['-', '=']).collect();
        let (label, rest) = (v[0], v[1]);
        let b = hash_string(label) as usize;

        // get the operation after the label
        match s.chars().nth(label.len()).unwrap() {
            '-' => {
                boxes[b].remove(label);
            }
            '=' => {
                let focal_length = rest[..].parse::<u8>().unwrap();
                boxes[b]
                    .entry(label)
                    .and_modify(|v| *v = focal_length)
                    .or_insert(focal_length);
            }
            _ => panic!("Invalid character {}", s),
        }
    }

    let mut total = 0;
    for (box_num, b) in boxes.iter().enumerate() {
        for (pos, (_, &value)) in b.iter().enumerate() {
            let focusing_power = (1 + box_num) * (pos + 1) * value as usize;
            total += focusing_power;
        }
    }
    Some(total as u32)
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
        assert_eq!(result, Some(145));
    }
}
