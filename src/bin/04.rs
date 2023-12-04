use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(4);

fn score_card(winning_numbers: Vec<i32>, card_numbers: Vec<i32>) -> u32 {
    let winning_numbers = count_winning_numbers(winning_numbers, card_numbers);
    // 1 point for first number, then double for every after that = 2 ^ (num - 1)
    if winning_numbers > 0 {
        2_u32.pow(winning_numbers - 1)
    } else {
        0
    }
}

fn count_winning_numbers(winning_numbers: Vec<i32>, card_numbers: Vec<i32>) -> u32 {
    card_numbers
        .iter()
        .filter(|v| winning_numbers.contains(v))
        .count() as u32
}

fn parse_card(line: &str) -> (Vec<i32>, Vec<i32>) {
    let (winning, card) = line.split(':').collect_vec()[1]
        .split('|')
        .collect_tuple()
        .unwrap();
    (
        winning
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect_vec(),
        card.split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect_vec(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    // Calculate the score for each card and then sum the total
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (winning_numbers, card_numbers) = parse_card(line);
                score_card(winning_numbers, card_numbers)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // Simplifying assumptions
    // - input cards are in order
    // - no cards extend beyond the list (per instructions)

    // Use a map to keep track of the count of each card number
    // (alternatively could use a vector with pre-determined size and index them since it's ordered and not sparse)
    let mut card_count = HashMap::new();

    for (i, card) in input.trim().split('\n').enumerate() {
        let card_num = i + 1;

        // Get the number of copies we already have of this card, including the original
        let copies = *card_count
            .entry(card_num)
            .and_modify(|v| *v += 1)
            .or_insert(1);

        let (winning_numbers, card_numbers) = parse_card(card);
        let c = count_winning_numbers(winning_numbers, card_numbers) as usize;
        // For each of the following cards (up to the limit of winning numbers),
        // add one for each copy we have
        for j in 1..=c {
            card_count
                .entry(card_num + j)
                .and_modify(|v| *v += copies)
                .or_insert(copies);
        }
    }

    Some(card_count.values().sum())
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
        assert_eq!(result, Some(30));
    }
}
