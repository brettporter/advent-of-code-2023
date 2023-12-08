use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(6);

fn calculate_distance(hold_duration: u64, total_time: u64) -> u64 {
    let remaining_time = total_time - hold_duration;
    remaining_time * hold_duration
}

fn find_num_records(time: u64, record: u64) -> u64 {
    // The results will be symmetrical over time, and once we have found a record distance,
    // they will continue being a record until reaching the same symmetrical point.
    //
    // Here we count the number of non-records starting at 0 to the midpoint by conducting a binary
    // search until we find the lowest distance that gives a record.
    //
    // The number of records will be the total time, less twice the number of non-records.

    let mut l = 0;
    let mut r = (time + 1) / 2;

    while l != r {
        if r - l == 1 {
            // avoid getting stuck on the boundary
            l = r;
            continue;
        }

        let m = (r - l) / 2 + l;
        if calculate_distance(m, time) > record {
            r = m;
        } else {
            l = m;
        }
    }
    time - l * 2 + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let p = parser!(
        line("Time:" string(" "+) repeat_sep(u64," "+))
        line("Distance:" string(" "+) repeat_sep(u64," "+))
    );
    let ((_, times), (_, records)) = p.parse(input).unwrap();

    // Enumerate the races and then multiply the number of records for each together
    Some(
        times
            .iter()
            .enumerate()
            .map(|(race, &time)| find_num_records(time, records[race]))
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let p = parser!(
        line("Time:" string(" "+) repeat_sep(string(digit+)," "+))
        line("Distance:" string(" "+) repeat_sep(string(digit+)," "+))
    );
    let ((_, times), (_, records)) = p.parse(input).unwrap();
    let (time, record) = (
        times.join("").parse().unwrap(),
        records.join("").parse().unwrap(),
    );

    Some(find_num_records(time, record))
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
