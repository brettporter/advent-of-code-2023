use itertools::Itertools;

advent_of_code::solution!(4);

fn score_card(winning_numbers: Vec<i32>, card_numbers: Vec<i32>) -> u32 {
    let winning_numbers = card_numbers
        .iter()
        .filter(|v| winning_numbers.contains(v))
        .count();
    if winning_numbers > 0 {
        2_u32.pow(winning_numbers as u32 - 1)
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (winning, card) = line.split(':').collect_vec()[1]
                    .split('|')
                    .collect_tuple()
                    .unwrap();
                let winning_numbers = winning
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec();
                let card_numbers = card
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec();
                score_card(winning_numbers, card_numbers)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            score_card(vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
            8
        );
        assert_eq!(
            score_card(
                vec![41, 92, 73, 84, 69],
                vec![59, 84, 76, 51, 58, 5, 54, 83]
            ),
            1
        );
        assert_eq!(
            score_card(
                vec![87, 83, 26, 28, 32],
                vec![88, 30, 70, 12, 93, 22, 82, 36]
            ),
            0
        );

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
