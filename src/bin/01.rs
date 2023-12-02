use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        // For each line, find all the numbers, then get the first and last in the line and combine them into a single two digit number
        // Sum the results of each line
        input
            .split('\n')
            .filter_map(|line| {
                let x: Vec<_> = line.chars().filter(|x| x.is_numeric()).collect();
                Some(x.first()?.to_digit(10).unwrap() * 10 + x.last()?.to_digit(10).unwrap())
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    Some(
        // Calculate the calibration values of each line and then sum the results
        input
            .split('\n')
            .filter_map(|line| {
                // Find the first of each named digit in a line and return its position and numerical value as a tuple, if one exists
                let text_digit_first_locations = digits
                    .iter()
                    .enumerate()
                    .filter_map(|(to, from)| line.find(*from).map(|v| (v, to as u32 + 1)));

                // Find the last of each named digit in a line and return its position and numerical value as a tuple, if one exists
                let text_digit_last_locations = digits
                    .iter()
                    .enumerate()
                    .filter_map(|(to, from)| line.rfind(*from).map(|v| (v, to as u32 + 1)));

                // Find all the numbers and return their position and numerical value as a tuple, then collect with the previous two lists into a single vector
                let digit_locations = line
                    .chars()
                    .enumerate()
                    .filter(|(_, v)| v.is_numeric())
                    .map(|(x, y)| (x, y.to_digit(10).unwrap()))
                    .chain(text_digit_first_locations)
                    .chain(text_digit_last_locations)
                    .collect_vec();

                // Find the numerical value with the minimum position and maxium position to calculate the calibration results
                Some(
                    digit_locations.iter().min_by_key(|x| x.0)?.1 * 10
                        + digit_locations.iter().max_by_key(|x| x.0)?.1,
                )
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result.unwrap(), 281);
    }
}
