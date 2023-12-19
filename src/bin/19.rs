use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(19);

#[derive(Debug)]
enum WorkflowRule {
    Rule(usize, usize, i32, String),
    Command(String),
}

const LT: usize = 0;
const GT: usize = 1;

fn execute_workflow(
    name: String,
    workflows: &HashMap<String, Vec<WorkflowRule>>,
    part: &Vec<i32>,
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
    // Accepted if reach end
    true
}

pub fn part_one(input: &str) -> Option<i32> {
    let workflow_rule = parser!(
        {
            cat:char_of("xmas") op:char_of("<>") value:i32 ":" workflow:string(alpha+) => WorkflowRule::Rule(cat, op, value, workflow),
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
                "{x=" i32 ",m=" i32 ",a=" i32 ",s=" i32 "}" // Currently assuming all present - can do char_of("xmas") and map later if not
            )
        )
    );

    let (workflows, parts) = p.parse(input).unwrap();

    let mut workflow_map = HashMap::new();
    for (name, rules) in workflows {
        workflow_map.insert(name, rules);
    }

    let mut accepted = Vec::new();

    for part in parts {
        let part_vec = vec![part.0, part.1, part.2, part.3];
        if execute_workflow(String::from("in"), &workflow_map, &part_vec) {
            accepted.push(part_vec);
        }
    }

    Some(accepted.iter().map(|part| part.iter().sum::<i32>()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
