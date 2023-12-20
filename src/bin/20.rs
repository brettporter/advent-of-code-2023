use core::panic;
use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

advent_of_code::solution!(20);

#[derive(PartialEq, Copy, Clone)]
enum Pulse {
    LOW,
    HIGH,
}

#[derive(Debug, Clone)]
enum ModuleType {
    BROADCASTER,
    FLIPFLOP,
    CONJUNCTION,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    mod_type: ModuleType,
    cables: Vec<String>,
}

fn parse_mapping(input: &str) -> IResult<&str, Module> {
    let (input, (t, name, _, dest)) = tuple((
        opt(one_of("&%")),
        alpha1,
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    ))(input)?;

    Ok((
        input,
        Module {
            name: name.to_string(),
            mod_type: match t {
                Some('%') => ModuleType::FLIPFLOP,
                Some('&') => ModuleType::CONJUNCTION,
                None => {
                    assert_eq!(name, "broadcaster");
                    ModuleType::BROADCASTER
                }
                _ => panic!(),
            },
            cables: dest.iter().map(|s| s.to_string()).collect(),
        },
    ))
}

fn parse(input: &str) -> Vec<Module> {
    let (_, mappings) = many1(terminated(parse_mapping, newline))(input).unwrap();
    mappings
}

pub fn part_one(input: &str) -> Option<u32> {
    // TODO: button module should not be pressed while still sending (hint for part 2?)
    // pulses sent in phases - send to a, b, c must finish c before the consequences of a are sent on

    // TODO: tests
    // Check single push against example in the doc
    // Repeat is same signal as all off
    // Check second example 4 times against the document, back to original state
    // Cycle detection?

    let modules = parse(input);
    let module_lookup = HashMap::<_, _>::from_iter(
        modules
            .iter()
            .enumerate()
            .map(|(i, v)| (v.name.to_owned(), i)),
    );

    // The list of inputs to each module
    let module_inputs: Vec<_> = modules
        .iter()
        .map(|m| {
            modules
                .iter()
                .filter_map(|v| v.cables.contains(&m.name).then_some(module_lookup[&v.name]))
                .collect::<Vec<_>>()
        })
        .collect();

    // 0/1 = on/off (flipflop), bitmask of connected modules low/high (conjunction)
    let mut state = vec![0; modules.len()];

    let (mut low_total, mut high_total) = (0, 0);

    for _ in 0..1000 {
        // TODO: memoise state? detect cycle for low/high increases?
        let (low, high) = push_button(&mut state, &modules, &module_lookup, &module_inputs);
        low_total += low;
        high_total += high;
    }

    Some(low_total * high_total)
}

fn push_button(
    state: &mut Vec<u64>,
    modules: &Vec<Module>,
    module_lookup: &HashMap<String, usize>,
    module_inputs: &Vec<Vec<usize>>,
) -> (u32, u32) {
    let mut queue = VecDeque::new();
    let (mut low, mut high) = (1, 0);

    // push the button
    queue.push_back((Pulse::LOW, usize::MAX, module_lookup["broadcaster"]));

    while let Some((pulse, from_idx, idx)) = queue.pop_front() {
        let module = &modules[idx];
        if let Some(send) = match module.mod_type {
            ModuleType::FLIPFLOP => {
                // Ignore high
                if pulse == Pulse::LOW {
                    // Toggle
                    state[idx] = 1 - state[idx];
                    Some(match state[idx] {
                        0 => Pulse::LOW,
                        1 => Pulse::HIGH,
                        _ => panic!(),
                    })
                } else {
                    None
                }
            }
            ModuleType::CONJUNCTION => {
                // update memory for that input
                state[idx] ^= 1 << from_idx;

                Some(
                    if module_inputs[idx]
                        .iter()
                        .all(|x| state[idx] & (1 << x) != 0)
                    {
                        Pulse::LOW
                    } else {
                        Pulse::HIGH
                    },
                )
            }
            ModuleType::BROADCASTER => Some(pulse),
        } {
            for c in &module.cables {
                match send {
                    Pulse::LOW => low += 1,
                    Pulse::HIGH => high += 1,
                }

                // Unknown labels can be ignored (e.g. output)
                if let Some(&dest_idx) = module_lookup.get(c) {
                    queue.push_back((send, idx, dest_idx));
                }
            }
        }
    }
    (low, high)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(32000000));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 12,
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
