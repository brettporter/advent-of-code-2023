use std::collections::HashMap;

advent_of_code::solution!(19);

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "19.pest"]
pub struct InputParser;

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

fn parse(input: &str) -> (Vec<Vec<usize>>, HashMap<String, Vec<WorkflowRule>>) {
    let file = InputParser::parse(Rule::file, input)
        .unwrap()
        .next()
        .unwrap();

    let mut workflow_map = HashMap::new();
    let mut parts = Vec::new();
    for r in file.into_inner() {
        match r.as_rule() {
            Rule::workflow => {
                let mut inner = r.into_inner();
                let id = inner.next().unwrap().as_str();
                let rules = inner.next().unwrap();
                let rules = rules
                    .into_inner()
                    .map(|rule| {
                        let rule = rule.into_inner().next().unwrap();
                        match rule.as_rule() {
                            Rule::condition => {
                                let mut i = rule.into_inner();
                                WorkflowRule::Rule(
                                    match i.next().unwrap().as_str() {
                                        "x" => 0,
                                        "m" => 1,
                                        "a" => 2,
                                        "s" => 3,
                                        _ => panic!(),
                                    },
                                    match i.next().unwrap().as_str() {
                                        "<" => Operation::LT,
                                        ">" => Operation::GT,
                                        _ => panic!(),
                                    },
                                    usize::from_str_radix(i.next().unwrap().as_str(), 10).unwrap(),
                                    i.next().unwrap().as_str().to_string(),
                                )
                            }
                            Rule::id => WorkflowRule::Command(rule.as_str().to_string()),
                            _ => panic!(),
                        }
                    })
                    .collect();
                workflow_map.insert(id.to_string(), rules);
            }
            Rule::bucket => {
                let v = r
                    .into_inner()
                    .map(|alloc| {
                        usize::from_str_radix(alloc.into_inner().nth(1).unwrap().as_str(), 10)
                            .unwrap()
                    })
                    .collect();
                parts.push(v);
            }
            _ => panic!(),
        }
    }

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
