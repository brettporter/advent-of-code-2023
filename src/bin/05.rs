use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(5);

struct Almanac {
    seeds: Vec<u64>,
    sections_dst: HashMap<String, String>,
    sections: HashMap<String, Vec<Mapping>>,
}

impl Almanac {
    fn map_to_location(&self, src: &str, src_value: u64) -> u64 {
        if src == "location" {
            return src_value;
        }
        let dst = &self.sections_dst[src];
        let dst_value = do_mapping(&self.sections[src], src_value);
        self.map_to_location(dst.as_str(), dst_value)
    }
}

#[derive(Debug)]
struct Mapping {
    dst_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

impl Mapping {
    fn new(dst_range_start: u64, src_range_start: u64, range_len: u64) -> Self {
        Self {
            dst_range_start,
            src_range_start,
            range_len,
        }
    }
}

fn do_mapping(mapping: &Vec<Mapping>, source: u64) -> u64 {
    for m in mapping {
        if (m.src_range_start..m.src_range_start + m.range_len).contains(&source) {
            return m.dst_range_start + (source - m.src_range_start);
        }
    }
    source
}

fn parse_maps(input: &str) -> Almanac {
    let mut sections_dst = HashMap::new();
    let mut sections = HashMap::new();

    let mut lines = input.trim().split('\n');
    let seeds = lines.next().unwrap()[7..]
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    lines.next();

    loop {
        if let Some(line) = lines.next() {
            let (from, _, to) = line[..line.len() - 5].split('-').collect_tuple().unwrap();
            let current_section = lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let mut numbers = line.split_ascii_whitespace();
                    Mapping::new(
                        numbers.next().unwrap().parse().unwrap(),
                        numbers.next().unwrap().parse().unwrap(),
                        numbers.next().unwrap().parse().unwrap(),
                    )
                })
                .collect_vec();
            sections_dst.insert(from.to_string(), to.to_string());
            sections.insert(from.to_string(), current_section);
        } else {
            break;
        }
    }

    Almanac {
        seeds,
        sections_dst,
        sections,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let almanac = parse_maps(input);

    almanac
        .seeds
        .iter()
        .map(|seed| almanac.map_to_location("seed", *seed))
        .min()
        .map(|x| x as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mapping = vec![Mapping::new(50, 98, 2), Mapping::new(52, 50, 48)];
        assert_eq!(do_mapping(&mapping, 79), 81);
        assert_eq!(do_mapping(&mapping, 14), 14);
        assert_eq!(do_mapping(&mapping, 55), 57);
        assert_eq!(do_mapping(&mapping, 13), 13);

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
