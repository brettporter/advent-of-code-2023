use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use range_collections::RangeSet2;

advent_of_code::solution!(5);

struct Almanac {
    seeds: Vec<u64>,
    sections: Vec<Vec<Mapping>>,
}

impl Almanac {
    fn map_to_location(&self, src_value: u64) -> u64 {
        // Traverse all the sections and map the value to get to the location from the seed
        let mut dst_value = src_value;
        for s in &self.sections {
            dst_value = do_mapping(s, dst_value);
        }
        dst_value
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

fn do_mapping(mapping: &Vec<Mapping>, src_value: u64) -> u64 {
    // Traverse the mappings inside the section. If a range matches, map that to the new destination.
    // Otherwise return the original value if no mappings match.
    for m in mapping {
        if (m.src_range_start..m.src_range_start + m.range_len).contains(&src_value) {
            return m.dst_range_start + (src_value - m.src_range_start);
        }
    }
    src_value
}

fn parse_maps(input: &str) -> Almanac {
    let p = parser!(
        line("seeds: " repeat_sep(u64, " "))
        line("")
        sections(
            line(string(alpha+) "-to-" string(alpha+) " map:")
            lines(u64 " " u64 " " u64)
        )
    );

    let (seeds, _, sec) = p.parse(input).unwrap();
    let sections = sec
        .iter()
        .map(|(_, s)| {
            s.iter()
                .map(|&(dst_range_start, src_range_start, range_len)| {
                    Mapping::new(dst_range_start, src_range_start, range_len)
                })
                .collect_vec()
        })
        .collect_vec();

    Almanac { seeds, sections }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = parse_maps(input);

    // For each seed, map it to the location based on the almanac, then determine the minimum value
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.map_to_location(*seed))
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = parse_maps(input);

    // Take the pairs from the seeds and turn them into ranges to being with
    let mut src = almanac
        .seeds
        .chunks(2)
        .map(|p| RangeSet2::from(p[0]..p[0] + p[1]))
        .collect_vec();

    for section in almanac.sections {
        let mut dst = Vec::new();
        while let Some(r) = src.pop() {
            // For each of the source ranges, go through the mapping and pass on the intersection to the next section offset by dest start.
            // Those that don't intersect, push back into the list and check it against the whole section
            // If for this source range there are no remaining intersections, pass on the original values
            let mut intersected = false;
            for mapping in &section {
                let mapping_src = RangeSet2::from(
                    mapping.src_range_start..mapping.src_range_start + mapping.range_len,
                );

                let intersect = r.intersection::<[u64; 2]>(&mapping_src);

                if !intersect.is_empty() {
                    let start = intersect.boundaries()[0] + mapping.dst_range_start
                        - mapping.src_range_start;
                    let end = intersect.boundaries()[1] + mapping.dst_range_start
                        - mapping.src_range_start;
                    dst.push(RangeSet2::from(start..end));

                    let difference = r.difference::<[u64; 2]>(&mapping_src);
                    for d in difference.iter() {
                        src.push(RangeSet2::from(d.cloned()));
                    }
                    intersected = true;
                    // start again with the new source intervals
                    break;
                }
            }
            if !intersected {
                dst.push(r);
            }
        }
        src = dst;
    }

    // For each of the final ranges, get the minimum of the lower bound on the range
    src.iter().map(|r| r.boundaries()[0]).min()
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
        assert_eq!(result, Some(46));
    }
}
