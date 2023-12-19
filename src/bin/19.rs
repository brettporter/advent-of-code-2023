use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

advent_of_code::solution!(19);

#[derive(Debug, Clone)]
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
    fn parse_condition(input: &str) -> IResult<&str, WorkflowRule> {
        let (input, (category, op, value, _, workflow)) =
            tuple((one_of("xmas"), one_of("<>"), u32, tag(":"), alpha1))(input)?;
        Ok((
            input,
            WorkflowRule::Rule(
                match category {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => panic!(),
                },
                match op {
                    '<' => Operation::LT,
                    '>' => Operation::GT,
                    _ => panic!(),
                },
                value as usize,
                workflow.to_string(),
            ),
        ))
    }

    fn parse_workflow(input: &str) -> IResult<&str, (String, Vec<WorkflowRule>)> {
        tuple((
            map(alpha1, |s: &str| s.to_string()),
            delimited(
                tag("{"),
                separated_list1(
                    tag(","),
                    alt((
                        parse_condition,
                        map(alpha1, |s: &str| WorkflowRule::Command(s.to_string())),
                    )),
                ),
                tag("}"),
            ),
        ))(input)
    }

    fn parse_parts(input: &str) -> IResult<&str, Vec<usize>> {
        let (input, parts) = delimited(
            tag("{"),
            separated_list1(tag(","), separated_pair(one_of("xmas"), tag("="), u32)),
            tag("}"),
        )(input)?;
        Ok((
            input,
            parts.iter().map(|(_, value)| *value as usize).collect(),
        )) // Currently assuming xmas order
    }

    let (_, (workflows, _, parts)) = tuple((
        many1(terminated(parse_workflow, newline)),
        newline,
        many1(terminated(parse_parts, opt(newline))),
    ))(input)
    .unwrap();
    (parts, HashMap::from_iter(workflows))
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
