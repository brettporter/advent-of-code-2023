use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(9);

fn solve_equation(equation: &[i32], backwards: bool) -> i32 {
    // Solve recursively by generating the diffs and then solving again,
    // finally returning the element to generate. This element is either
    // added to the last element to append to the list and returned up,
    // or subtracted from the first element to prepend to the list and
    // returned up.

    // If all the elements are 0 we are done and return the new 0
    // element to help generate the next difference.
    if equation.iter().all(|i| *i == 0) {
        return 0;
    }

    let mut diffs = Vec::new();
    for i in 1..equation.len() {
        diffs.push(equation[i] - equation[i - 1]);
    }

    let result = solve_equation(&diffs, backwards);
    if backwards {
        equation.first().unwrap() - result
    } else {
        equation.last().unwrap() + result
    }
}

fn solve(input: &str, backwards: bool) -> Option<i32> {
    let p = parser!(lines(line(repeat_sep(i32, " "))));
    let v = p.parse(input).unwrap();

    // Solve for the new element to generate in each equation, and then add them together
    Some(
        v.iter()
            .map(|equation| solve_equation(equation, backwards))
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<i32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<i32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
