use std::cmp::Ordering;

use freqdist::FrequencyDistribution;
use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, PartialOrd, Debug)]
enum HandCount {
    Nothing,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, Debug)]
struct Hand {
    value: Vec<u8>,
    rank: HandCount,
}

impl Hand {
    fn new(s: &str) -> Self {
        let rank = calc_rank(s);

        Self {
            value: String::from(s).chars().map(|c| card_value(c)).collect_vec(),
            rank,
        }
    }
}

fn card_value(c: char) -> u8 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card value {}", c),
    }
}

fn calc_rank(s: &str) -> HandCount {
    let mut f = FrequencyDistribution::<char>::new();
    for c in s.chars() {
        f.insert(c);
    }

    let combos = f.iter_non_zero().map(|k| f.get(k)).sorted().collect_vec();

    match combos.as_slice() {
        [5] => HandCount::FiveOfAKind,
        [1, 4] => HandCount::FourOfAKind,
        [2, 3] => HandCount::FullHouse,
        [1, 1, 3] => HandCount::ThreeOfAKind,
        [1, 2, 2] => HandCount::TwoPair,
        [1, 1, 1, 2] => HandCount::OnePair,
        _ => HandCount::Nothing,
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result = self.rank.partial_cmp(&other.rank).unwrap();
        if result == Ordering::Equal {
            Some(self.value.cmp(&other.value))
        } else {
            Some(result)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank.eq(&other.rank)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let bids = input
        .trim()
        .split('\n')
        .map(|line| line.split_ascii_whitespace().collect_vec())
        .sorted_by_key(|v| Hand::new(v[0]));

    Some(
        bids.enumerate()
            .map(|(i, bid)| (i as u32 + 1) * bid[1].parse::<u32>().unwrap())
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
        assert!(Hand::new("AAAAA") > Hand::new("AA8AA")); // 5 of a kind
        assert!(Hand::new("AA8AA") > Hand::new("23332")); // 4 of a kind
        assert!(Hand::new("23332") > Hand::new("TTT98")); // full house
        assert!(Hand::new("TTT98") > Hand::new("23432")); // three of a kind
        assert!(Hand::new("23432") > Hand::new("A23A4")); // two pair
        assert!(Hand::new("A23A4") > Hand::new("23456")); // one pair

        assert!(Hand::new("33332") > Hand::new("2AAAA")); // stronger card
        assert!(Hand::new("77888") > Hand::new("77788")); // stronger card

        assert_eq!(
            vec![
                Hand::new("32T3K"),
                Hand::new("T55J5"),
                Hand::new("KK677"),
                Hand::new("KTJJT"),
                Hand::new("QQQJA")
            ]
            .into_iter()
            .sorted()
            .collect_vec(),
            vec![
                Hand::new("32T3K"),
                Hand::new("KTJJT"),
                Hand::new("KK677"),
                Hand::new("T55J5"),
                Hand::new("QQQJA")
            ]
        );

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
