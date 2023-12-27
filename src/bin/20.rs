use core::panic;
use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};
use num::Integer;
use rustc_hash::FxHashMap;

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

struct Machine {
    modules: Vec<Module>,
    module_lookup: FxHashMap<String, usize>,
    module_inputs: Vec<Vec<usize>>,
    state: Vec<u64>,
}

impl Machine {
    fn setup(input: &str) -> Self {
        let modules = parse(input);

        // Map module names to index number, so we can reference modules by index rather than name
        let module_lookup = FxHashMap::<_, _>::from_iter(
            modules
                .iter()
                .enumerate()
                .map(|(i, v)| (v.name.to_owned(), i)),
        );

        // The list of inputs to each module
        // Map the destination module to the index of the source module
        let module_inputs = modules
            .iter()
            .map(|m| {
                modules
                    .iter()
                    .filter_map(|v| v.cables.contains(&m.name).then_some(module_lookup[&v.name]))
                    .collect()
            })
            .collect();

        // State will be 0/1 = on/off (flipflop), bitmask of connected modules low/high as 0/1 (conjunction)
        let state = vec![0; modules.len()];

        Self {
            modules,
            module_lookup,
            module_inputs,
            state,
        }
    }

    fn push_button<F>(&mut self, emit_pulses: &mut F)
    where
        F: FnMut(&Module, Pulse),
    {
        // Pulses still to process for this step. Contains the pulse, from module (index), to module (index)
        let mut queue = VecDeque::new();

        // push the button, send a low pulse to the broadcaster
        queue.push_back((Pulse::LOW, usize::MAX, self.module_lookup["broadcaster"]));

        // Process any pulses still remaining in this step
        while let Some((pulse, from_idx, idx)) = queue.pop_front() {
            let module = &self.modules[idx];
            if let Some(send) = match module.mod_type {
                ModuleType::FLIPFLOP => {
                    if pulse == Pulse::LOW {
                        // Toggle state
                        self.state[idx] = 1 - self.state[idx];
                        // Return the new state as a pulse to send on
                        Some(match self.state[idx] {
                            0 => Pulse::LOW,
                            1 => Pulse::HIGH,
                            _ => unreachable!(),
                        })
                    } else {
                        // Ignore high pulses to flipflop - no pulse
                        None
                    }
                }
                ModuleType::CONJUNCTION => {
                    // update memory for that input. First clear bit, then set if high.
                    self.state[idx] &= !(1 << from_idx);
                    if pulse == Pulse::HIGH {
                        self.state[idx] |= 1 << from_idx;
                    }

                    // If all inputs remember high (set to 1), send a low pulse, otherwise send high
                    Some(
                        if self.module_inputs[idx]
                            .iter()
                            .all(|x| self.state[idx] & (1 << x) != 0)
                        {
                            Pulse::LOW
                        } else {
                            Pulse::HIGH
                        },
                    )
                }
                // Passthrough
                ModuleType::BROADCASTER => Some(pulse),
            } {
                // Emit a pulses from this module to each of the cables
                emit_pulses(module, send);

                // Emit pulse to each of the destination modules
                for c in &module.cables {
                    // Unknown labels (e.g. output) do not need to be processed further
                    if let Some(&dest_idx) = self.module_lookup.get(c) {
                        // Add to queue to process from this module to the destination
                        queue.push_back((send, idx, dest_idx));
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut machine = Machine::setup(input);

    let (mut low_total, mut high_total) = (0, 0);

    for _ in 0..1000 {
        low_total += 1; // add one for the button push
        machine.push_button(&mut |module, send| match send {
            Pulse::LOW => low_total += module.cables.len() as u32,
            Pulse::HIGH => high_total += module.cables.len() as u32,
        });
    }

    Some(low_total * high_total)
}

fn _print_state(state: &Vec<u64>, modules: &Vec<Module>, value_keys: &mut FxHashMap<u64, u16>) {
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
    let mut machine = Machine::setup(input);

    // Examined input, found one conjunction in front of rx
    // To send low to rx, this gate must get high from all inputs
    let start = machine
        .modules
        .iter()
        .find(|m| m.cables.contains(&"rx".to_string()))
        .unwrap();
    assert_eq!(start.mod_type, ModuleType::CONJUNCTION);

    // Get the conjunctions that are in front of the above
    // Looking for cycle when all these send high at the same time
    let gates = machine
        .modules
        .iter()
        .filter_map(|m| m.cables.contains(&start.name).then_some(m.name.to_owned()))
        .collect::<Vec<_>>();

    // Find the first cycle length for each of the listed gates
    // Note we have an assumption that they have a consistent cycle from 0 based on the input
    let mut gate_cycles = FxHashMap::default();
    let mut gates_cycled = 0;
    let mut step = 1;
    while gates_cycled < gates.len() {
        machine.push_button(&mut |module, send| {
            if module.mod_type == ModuleType::CONJUNCTION && send == Pulse::HIGH {
                if !gate_cycles.contains_key(&module.name) {
                    gate_cycles.insert(module.name.clone(), step);
                    if gates.contains(&module.name) {
                        gates_cycled += 1;
                    }
                }
            }
        });
        step += 1;
    }

    // Calculate the LCM of all the cycle lengths to determine when all of those gates will be HIGH
    // simultaneously, which will result in LOW to rx
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
