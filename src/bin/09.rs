use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(9);

fn solve_equation(equation: &[i32]) -> i32 {
    println!("Solving {:?}", equation);
    if equation.iter().all(|i| *i == 0) {
        println!("Got 0");
        return 0;
    }

    let mut diffs = Vec::new();
    for i in 1..equation.len() {
        diffs.push(equation[i] - equation[i - 1]);
    }

    let result = solve_equation(&diffs);
    println!(
        "Got {result} {} for {:?}",
        equation.last().unwrap() + result,
        equation
    );
    equation.last().unwrap() + result
}

pub fn part_one(input: &str) -> Option<i32> {
    let p = parser!(lines(line(repeat_sep(i32, " "))));
    let v = p.parse(input).unwrap();

    Some(v.iter().map(|equation| solve_equation(equation)).sum())
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
