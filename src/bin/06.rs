use itertools::Itertools;

advent_of_code::solution!(6);

fn calculate_distance(hold_duration: u64, total_time: u64) -> u64 {
    let remaining_time = total_time - hold_duration;
    remaining_time * hold_duration
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.split('\n');
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    let mut result = 1;
    for (race, time) in times.iter().enumerate() {
        let record = distances[race];
        result *= (0..=*time)
            .filter(|hold_duration| {
                let distance = calculate_distance(*hold_duration, *time);
                distance > record
            })
            .count() as u32;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.split('\n');
    let time = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<u64>()
        .unwrap();
    let record = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<u64>()
        .unwrap();

    let mut result = 1;
    result *= (0..=time)
        .filter(|hold_duration| {
            let distance = calculate_distance(*hold_duration, time);
            distance > record
        })
        .count() as u32;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(calculate_distance(0, 7), 0);
        assert_eq!(calculate_distance(1, 7), 6);
        assert_eq!(calculate_distance(2, 7), 10);
        assert_eq!(calculate_distance(3, 7), 12);
        assert_eq!(calculate_distance(4, 7), 12);
        assert_eq!(calculate_distance(5, 7), 10);
        assert_eq!(calculate_distance(6, 7), 6);
        assert_eq!(calculate_distance(7, 7), 0);

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
