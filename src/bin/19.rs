use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(19);

#[derive(Debug)]
enum WorkflowRule {
    Rule(usize, usize, usize, String),
    Command(String),
}

const LT: usize = 0;
const GT: usize = 1;

fn execute_workflow(
    name: String,
    workflows: &HashMap<String, Vec<WorkflowRule>>,
    part: &Vec<usize>,
) -> bool {
    if name == "R" {
        return false;
    }
    if name == "A" {
        return true;
    }

    for rule in workflows.get(&name).unwrap() {
        match rule {
            WorkflowRule::Rule(cat, op, value, workflow) => {
                let eval = match *op {
                    LT => part[*cat] < *value,
                    GT => part[*cat] > *value,
                    _ => unreachable!(),
                };
                if eval {
                    return execute_workflow(workflow.to_owned(), workflows, part);
                }
            }
            WorkflowRule::Command(cmd) => return execute_workflow(cmd.to_owned(), workflows, part),
        }
    }
    unreachable!()
}

fn parse(
    input: &str,
) -> (
    Vec<(usize, usize, usize, usize)>,
    HashMap<String, Vec<WorkflowRule>>,
) {
    let workflow_rule = parser!(
        {
            cat:char_of("xmas") op:char_of("<>") value:usize ":" workflow:string(alpha+) => WorkflowRule::Rule(cat, op, value, workflow),
            workflow:string(alpha+) => WorkflowRule::Command(workflow),
        }
    );

    let p = parser!(
        section(
            lines(
                workflow:string(alpha+) "{" rules:repeat_sep(workflow_rule, ",") "}"
            )
        )
        section(
            lines(
                "{x=" usize ",m=" usize ",a=" usize ",s=" usize "}" // Currently assuming all present - can do char_of("xmas") and map later if not
            )
        )
    );

    let (workflows, parts) = p.parse(input).unwrap();

    let mut workflow_map = HashMap::new();
    for (name, rules) in workflows {
        workflow_map.insert(name, rules);
    }
    (parts, workflow_map)
}

fn count_accepted_combinations(
    segments: Vec<(usize, usize)>,
    name: String,
    workflow_map: &HashMap<String, Vec<WorkflowRule>>,
) -> usize {
    if name == "R" {
        return 0;
    }
    if name == "A" {
        return segments
            .iter()
            .map(|s| s.1 - s.0 + 1)
            .reduce(|acc, e| acc * e)
            .unwrap();
    }

    let mut total = 0;
    let mut remaining = segments;
    for rule in workflow_map.get(&name).unwrap() {
        match rule {
            WorkflowRule::Rule(cat, op, value, workflow) => {
                let (wf_seg, rem_seg) = match *op {
                    LT => ((remaining[*cat].0, *value - 1), (*value, remaining[*cat].1)),
                    GT => ((*value + 1, remaining[*cat].1), (remaining[*cat].0, *value)),
                    _ => unreachable!(),
                };
                let mut wf = remaining.clone();
                wf[*cat] = wf_seg;
                total += count_accepted_combinations(wf, workflow.to_owned(), workflow_map);

                remaining = remaining.clone();
                remaining[*cat] = rem_seg;
            }
            WorkflowRule::Command(cmd) => {
                return total + count_accepted_combinations(remaining, cmd.to_owned(), workflow_map)
            }
        }
    }

    unreachable!()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (parts, workflow_map) = parse(input);

    let mut accepted = Vec::new();

    for part in parts {
        let part_vec = vec![part.0, part.1, part.2, part.3];
        if execute_workflow(String::from("in"), &workflow_map, &part_vec) {
            accepted.push(part_vec);
        }
    }

    Some(accepted.iter().map(|part| part.iter().sum::<usize>()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, workflow_map) = parse(input);

    let segments = vec![(1, 4000); 4];

    Some(count_accepted_combinations(
        segments,
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
