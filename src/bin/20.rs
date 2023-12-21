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

#[derive(PartialEq, Copy, Clone, Debug)]
enum Pulse {
    LOW,
    HIGH,
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleType {
    BROADCASTER,
    FLIPFLOP,    // NOT gate
    CONJUNCTION, // NAND gate
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

fn setup_machine(
    input: &str,
) -> (
    Vec<Module>,
    HashMap<String, usize>,
    Vec<Vec<usize>>,
    Vec<u64>,
) {
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
    let state = vec![0; modules.len()];
    (modules, module_lookup, module_inputs, state)
}

fn push_button(
    state: &mut Vec<u64>,
    modules: &Vec<Module>,
    module_lookup: &HashMap<String, usize>,
    module_inputs: &Vec<Vec<usize>>,
) -> (u32, u32, u32) {
    let mut queue = VecDeque::new();
    let (mut low, mut high, mut rx_low_pulses) = (1, 0, 0);

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
                // update memory for that input. First clear bit, then set if high.
                // TODO: unit test - part 1 passes if this is wrong but input doesn't
                state[idx] &= !(1 << from_idx);
                if pulse == Pulse::HIGH {
                    state[idx] |= 1 << from_idx;
                }

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
            match send {
                Pulse::LOW => low += module.cables.len() as u32,
                Pulse::HIGH => high += module.cables.len() as u32,
            }
            for c in &module.cables {
                if c == "rx" {
                    // TODO: do we ignore high, or does that also make it invalid?
                    if send == Pulse::LOW {
                        rx_low_pulses += 1;
                    }
                }

                // Unknown labels can be ignored (e.g. output)
                if let Some(&dest_idx) = module_lookup.get(c) {
                    queue.push_back((send, idx, dest_idx));
                }
            }
        }
    }
    (low, high, rx_low_pulses)
}

pub fn part_one(input: &str) -> Option<u32> {
    // TODO: button module should not be pressed while still sending (hint for part 2?)
    // pulses sent in phases - send to a, b, c must finish c before the consequences of a are sent on

    // TODO: tests
    // Check single push against example in the doc
    // Repeat is same signal as all off
    // Check second example 4 times against the document, back to original state
    // Cycle detection?

    let (modules, module_lookup, module_inputs, mut state) = setup_machine(input);

    let (mut low_total, mut high_total) = (0, 0);

    let mut value_keys = HashMap::new();

    for m in &modules {
        print!("{} ", &m.name[0..2]);
    }
    println!();
    for m in &modules {
        print!(
            " {} ",
            match m.mod_type {
                ModuleType::BROADCASTER => "B",
                ModuleType::FLIPFLOP => "F",
                ModuleType::CONJUNCTION => "C",
            }
        );
    }
    println!();

    for _ in 0..1000 {
        // TODO: memoise state? detect cycle for low/high increases?
        let (low, high, _) = push_button(&mut state, &modules, &module_lookup, &module_inputs);

        print_state(&state, &modules, &mut value_keys);

        low_total += low;
        high_total += high;
    }

    println!();
    println!("Keys:");
    for (v, k) in value_keys {
        let sv: Vec<_> = modules
            .iter()
            .enumerate()
            .filter_map(|(i, m)| (v & (1 << i) != 0).then_some(&m.name))
            .collect();

        // TODO: why is k repeated?
        println!("  {:2x} => {:?}", k, sv);
    }

    Some(low_total * high_total)
}

fn print_state(state: &Vec<u64>, modules: &Vec<Module>, value_keys: &mut HashMap<u64, u8>) {
    for (i, v) in state.iter().enumerate() {
        if modules[i].mod_type == ModuleType::CONJUNCTION {
            let next = value_keys.len() as u8;
            let c = value_keys.entry(*v).or_insert(next);
            print!("{:2x} ", c);
        } else {
            print!("{:2} ", v);
        }
    }
    println!();
}

pub fn part_two(input: &str) -> Option<u32> {
    let (modules, module_lookup, module_inputs, mut state) = setup_machine(input);

    // for rx to be sent 0, one of it's inputs must send 0
    // for inputs
    //  if a not gate, one of it's inputs must send !v
    //  if a nand gate, if it is 0, all of it's inputs must send 1 (but can be state reliant)
    //  if a nand gate, if it is 1, one of it's inputs must send 0
    //  if it is broadcast, discard if 1 because button sends 0
    // how many valid combos are there?
    // what is the required state to trigger the 1? should I be evaluating this as it goes?
    // how do I figure out the shortest path to that state?

    // Patterns (up to 1000):
    //  9 NAND gates, rest are NOT
    //  Looks like xc and gh has a period of 4.
    //  Looks like bh (1), jf (2), mf (3), sh (5), mz (7) is constant
    //  cn, hz have period of 15
    //  Note those with period have certain number of repeats before changing.
    //  Some of the changes are not a full period, there's one value in between

    let start = modules
        .iter()
        .find(|m| m.cables.contains(&"rx".to_string()))
        .unwrap();

    println!(
        "g = {:?} {:?}",
        start, module_inputs[module_lookup[&start.name]]
    );
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
}
