use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(19);

#[derive(Debug)]
enum Operation {
    LT,
    GT,
}

#[derive(Debug)]
enum WorkflowRule {
    Rule(usize, Operation, usize, String),
    Command(String),
}

fn parse(
    input: &str,
) -> (
    Vec<(usize, usize, usize, usize)>,
    HashMap<String, Vec<WorkflowRule>>,
) {
    let mut workflow_map = HashMap::new();

    let wf_regex = Regex::new(r"([a-z]+)\{(.*)\}").unwrap();
    let wf_rule_regex = Regex::new(r"([xmas])([<>])([0-9]+):([a-z]+|A|R)").unwrap();
    let part_regex = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();

    for c in wf_regex.captures_iter(input) {
        let (_, [name, r]) = c.extract();
        let rules = r
            .split(",")
            .map(|r| {
                if let Some(c) = wf_rule_regex.captures(r) {
                    let (_, [part, op, value, workflow]) = c.extract();
                    WorkflowRule::Rule(
                        "xmas"
                            .chars()
                            .position(|c| c == part.chars().nth(0).unwrap())
                            .unwrap(),
                        match op {
                            "<" => Operation::LT,
                            ">" => Operation::GT,
                            _ => panic!("Invalid input operation"),
                        },
                        usize::from_str_radix(value, 10).unwrap(),
                        workflow.to_string(),
                    )
                } else {
                    WorkflowRule::Command(r.to_string())
                }
            })
            .collect();
        workflow_map.insert(name.to_string(), rules);
    }

    let parts = part_regex
        .captures_iter(input)
        .map(|c| {
            let (_, [x, m, a, s]) = c.extract();
            (
                x.parse().unwrap(),
                m.parse().unwrap(),
                a.parse().unwrap(),
                s.parse().unwrap(),
            )
        })
        .collect();

    (parts, workflow_map)
}

fn execute_workflow(
    name: String,
    workflows: &HashMap<String, Vec<WorkflowRule>>,
    part: &Vec<usize>,
) -> bool {
    // If the workflow we reached is R - return false (rejected)
    if name == "R" {
        return false;
    }
    // If the workflow we reached is R - return false (accepted)
    if name == "A" {
        return true;
    }

    // Otherwise, lookup the workflow by name and process the rules in order
    for rule in workflows.get(&name).unwrap() {
        match rule {
            WorkflowRule::Rule(cat, op, value, workflow) => {
                // Conditional rule - test if the corresponding part category matches the rule
                // If so, run the given workflow (and do not continue processing rules).
                // If not, continue processing further rules
                let eval = match *op {
                    Operation::LT => part[*cat] < *value,
                    Operation::GT => part[*cat] > *value,
                };
                if eval {
                    return execute_workflow(workflow.to_owned(), workflows, part);
                }
            }
            // Workflow only - run the given workflow and stop processing rules
            WorkflowRule::Command(cmd) => return execute_workflow(cmd.to_owned(), workflows, part),
        }
    }
    unreachable!()
}

fn count_accepted_combinations(
    segments: Vec<(usize, usize)>,
    name: String,
    workflow_map: &HashMap<String, Vec<WorkflowRule>>,
) -> usize {
    // If this workflow path is rejected, then none of the segments will be accepted - return 0
    if name == "R" {
        return 0;
    }
    // If this workflow path is accepted, count the total combinations for the given segments
    // Multiply possible x * m * a * s to get all combinations
    if name == "A" {
        return segments
            .iter()
            .map(|s| s.1 - s.0 + 1)
            .reduce(|acc, e| acc * e)
            .unwrap();
    }

    // Otherwise, find the named workflow and process the rules in order given the segments
    let mut total = 0;
    let mut remaining = segments;
    for rule in workflow_map.get(&name).unwrap() {
        match rule {
            WorkflowRule::Rule(cat, op, value, workflow) => {
                // Conditional rule. Split the segment for the given category into two
                // The successful set goes to the workflow, and the unsuccessful set continues on
                // to further rules (remaining)
                let (wf_seg, rem_seg) = match *op {
                    Operation::LT => ((remaining[*cat].0, *value - 1), (*value, remaining[*cat].1)),
                    Operation::GT => ((*value + 1, remaining[*cat].1), (remaining[*cat].0, *value)),
                };
                // Take the remaining segments and replace one segment to pass to the workflow
                let mut wf = remaining.clone();
                wf[*cat] = wf_seg;
                // Add the total of that path to the running total for this workflow, but do not continue
                // checking this segment in further rules.
                total += count_accepted_combinations(wf, workflow.to_owned(), workflow_map);

                // Replace the remaining segment for this category with those not matching the condition
                // Continue to further rules
                remaining[*cat] = rem_seg;
            }
            WorkflowRule::Command(cmd) => {
                // Add the total of that path to the running total for this workflow, and then return the total
                // instead of checking further rules
                return total
                    + count_accepted_combinations(remaining, cmd.to_owned(), workflow_map);
            }
        }
    }

    unreachable!()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (parts, workflow_map) = parse(input);

    // Find all the parts that are accepted by eecuting the workflow for that part
    // Sum the elements of the part (ratings), and then return the sum of this across all parts
    Some(
        parts
            .iter()
            .map(|part| vec![part.0, part.1, part.2, part.3])
            .filter(|part| execute_workflow(String::from("in"), &workflow_map, &part))
            .map(|part| part.iter().sum::<usize>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, workflow_map) = parse(input);

    // Traverse the workflows counting accepted combinations across the segments
    // starting with 1 - 4000 for each
    Some(count_accepted_combinations(
        vec![(1, 4000); 4],
        String::from("in"),
        &workflow_map,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
