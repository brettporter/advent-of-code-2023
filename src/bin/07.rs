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
        'j' => 1, // joker has lowest value
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

    // Go through the calculated frequency distribution of each card type that is present,
    // remove jokers, and sort in descending order
    let mut combos = f
        .iter_non_zero()
        .filter(|&k| *k != 'j')
        .map(|k| f.get(k))
        .sorted()
        .rev()
        .collect_vec();

    if combos.is_empty() {
        // Special case: 5 jokers
        return HandCount::FiveOfAKind;
    }

    // Add jokers to the highest frequency card, this will always result in best hand
    combos[0] += f.get(&'j');

    match combos.as_slice() {
        [5] => HandCount::FiveOfAKind,
        [4, 1] => HandCount::FourOfAKind,
        [3, 2] => HandCount::FullHouse,
        [3, 1, 1] => HandCount::ThreeOfAKind,
        [2, 2, 1] => HandCount::TwoPair,
        [2, 1, 1, 1] => HandCount::OnePair,
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
        // Compare hands by rank
        let result = self.rank.partial_cmp(&other.rank).unwrap();
        if result == Ordering::Equal {
            Some(self.value.cmp(&other.value))
        } else {
            Some(result)
        }
    }
}

impl PartialEq for Hand {
    // Compare hands by rank
    fn eq(&self, other: &Self) -> bool {
        self.rank.eq(&other.rank)
    }
}

fn calculate_result(input: &str, with_joker: bool) -> usize {
    // read hand and bid, then sort by rank low -> high
    let bids = input
        .trim()
        .split('\n')
        .map(|line| line.split_ascii_whitespace().collect_vec())
        .sorted_by_key(|v| {
            let hand = v[0];
            if with_joker {
                Hand::new(hand.replace('J', "j").as_str())
            } else {
                Hand::new(hand)
            }
        });

    // multiply rank (i + 1) by the bid for each hand, then return the sum
    let result = bids
        .enumerate()
        .map(|(i, bid)| (i + 1) * bid[1].parse::<usize>().unwrap())
        .sum();
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(calculate_result(input, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(calculate_result(input, true))
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
        assert_eq!(Hand::new("QjjQ2").rank, HandCount::FourOfAKind);

        assert!(Hand::new("QQQQ2") > Hand::new("jKKK2")); // 4 of a kind - stronger card

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
