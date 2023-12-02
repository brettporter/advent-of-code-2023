use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
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

    let new_input = input
        .split('\n')
        .map(|line| {
            let mut new_line = line.to_string();

            loop {
                match digits
                    .iter()
                    .enumerate()
                    .filter_map(|(to, from)| new_line.find(*from).map(|v| (from, to + 1, v)))
                    .min_by_key(|x| x.2)
                {
                    Some(sub) => {
                        new_line = new_line.replacen(sub.0, sub.1.to_string().as_str(), 1);
                    }
                    None => break,
                };
            }

            new_line
        })
        .join("\n");

    part_one(&new_input)
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
