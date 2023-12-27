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
use num::Integer;

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

fn push_button<F>(
    state: &mut Vec<u64>,
    modules: &Vec<Module>,
    module_lookup: &HashMap<String, usize>,
    module_inputs: &Vec<Vec<usize>>,
    check_gate: &mut F,
) where
    F: FnMut(&Module, Pulse),
{
    let mut queue = VecDeque::new();

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
            check_gate(module, send);

            for c in &module.cables {
                // Unknown labels can be ignored (e.g. output)
                if let Some(&dest_idx) = module_lookup.get(c) {
                    queue.push_back((send, idx, dest_idx));
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (modules, module_lookup, module_inputs, mut state) = setup_machine(input);

    let (mut low_total, mut high_total) = (0, 0);

    for _ in 0..1000 {
        low_total += 1; // add one for the button
        push_button(
            &mut state,
            &modules,
            &module_lookup,
            &module_inputs,
            &mut |module, send| match send {
                Pulse::LOW => low_total += module.cables.len() as u32,
                Pulse::HIGH => high_total += module.cables.len() as u32,
            },
        );
    }

    Some(low_total * high_total)
}

fn _print_state(state: &Vec<u64>, modules: &Vec<Module>, value_keys: &mut HashMap<u64, u16>) {
    for (i, v) in state.iter().enumerate() {
        if modules[i].mod_type == ModuleType::CONJUNCTION {
            let next = value_keys.len() as u16;
            let c = value_keys.entry(*v).or_insert(next);
            print!("{:3x} ", c);
        } else {
            print!("{:3} ", v);
        }
    }
    println!();
}

pub fn part_two(input: &str) -> Option<usize> {
    let (modules, module_lookup, module_inputs, mut state) = setup_machine(input);

    // Examined input, found one conjunction in front of rx
    // To send low to rx, this gate must get high from all inputs
    let start = modules
        .iter()
        .find(|m| m.cables.contains(&"rx".to_string()))
        .unwrap();
    assert_eq!(start.mod_type, ModuleType::CONJUNCTION);

    // Get the conjunctions that are in front of the above
    // Looking for cycle when all these send high at the same time
    let gates = modules
        .iter()
        .filter_map(|m| m.cables.contains(&start.name).then_some(m.name.to_owned()))
        .collect::<Vec<_>>();

    let mut gate_cycles = HashMap::new();
    let mut gates_cycled = 0;
    let mut step = 1;
    while gates_cycled < gates.len() {
        push_button(
            &mut state,
            &modules,
            &module_lookup,
            &module_inputs,
            &mut |module, send| {
                if module.mod_type == ModuleType::CONJUNCTION && send == Pulse::HIGH {
                    if !gate_cycles.contains_key(&module.name) {
                        gate_cycles.insert(module.name.clone(), step);
                        if gates.contains(&module.name) {
                            gates_cycled += 1;
                        }
                    }
                }
            },
        );
        step += 1;
    }

    gate_cycles
        .values()
        .map(|v| *v)
        .reduce(|acc, e| acc.lcm(&e))
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
