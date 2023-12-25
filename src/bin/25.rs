use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, connections) = parse_input(input).unwrap();

    println!("{}", "graph {");
    for (from, _, to) in connections {
        for t in to {
            println!("{from} -- {t};");
        }
    }
    println!("{}", "}");

    None
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, &str, Vec<&str>)>> {
    many1(terminated(
        tuple((alpha1, tag(": "), separated_list1(space1, alpha1))),
        opt(newline),
    ))(input)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
