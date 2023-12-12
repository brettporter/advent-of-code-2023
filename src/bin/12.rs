use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(12);

fn find_arrangements(str: &str, sizes: &[i32]) -> i32 {
    let total = sizes.iter().sum::<i32>();
    let l = str.len();
    let missing = str.len() as i32 - sizes.iter().sum::<i32>();
    let to_allocate = missing - (sizes.len() as i32 - 1);
    let first = 0;
    let last = sizes.len();    

    let mut combos = Vec::new();
    for i in 0..=to_allocate {
        combos.push(vec![i]);
    }
    for pos in 1..=last {
        let mut new_combos = Vec::new();
        for c in &combos {
            let used = c.iter().sum::<i32>();
            if pos < last {
                for i in 0..=(to_allocate - used) {
                    let mut new_c = c.clone();
                    new_c.push(i);
                    new_combos.push(new_c);
                }
            } else {
                let mut new_c = c.clone();
                new_c.push(to_allocate - used);
                new_combos.push(new_c);
            }
        }
        combos = new_combos;
    }

    // TODO: need to build the combos faster, too slow. Maybe check as we go?
    println!("combos = {:?}", combos.len());

    let chr = str.as_bytes();
    let mut arrangements = 0;
    'outer: for c in combos {
        let mut pos = 0;
        let mut valid = true;
        assert_eq!(c.len(), sizes.len() + 1);

        for i in 0..sizes.len() {
            let expected = if i == 0 { c[i] } else { c[i] + 1 };
            for j in 0..expected {
                if chr[pos] != b'.' && chr[pos] != b'?' {
                    valid = false;
                    continue 'outer;
                }
                pos += 1;
            }
            for j in 0..sizes[i] {
                if chr[pos] != b'#' && chr[pos] != b'?' {
                    valid = false;
                    continue 'outer;
                }
                pos += 1;
            }
        }
        for i in 0..*c.last().unwrap() {
            if chr[pos] != b'.' && chr[pos] != b'?' {
                valid = false;
                continue 'outer;
            }
            pos += 1;
        }
        println!("{:?} is {valid}", c);
        if valid {
            for i in 0..sizes.len() {
                for j in 0..c[i] {
                    print!(".");
                }
                for j in 0..sizes[i] {
                    print!("#");
                }
            }
            for j in 0..*c.last().unwrap() {
                print!(".");
            }
            println!();
            arrangements += 1;
        }
    }

    println!("Found {arrangements}");

    arrangements
}

pub fn part_one(input: &str) -> Option<i32> {
    let p = parser!(lines(
        // char_of(".#?")+ string(" ") repeat_sep(i32, ",")
        string(any_char+) string(" ") repeat_sep(i32, ",")
    ));

    let v = p.parse(input).unwrap();

    Some(
        v.iter()
            .map(|line| find_arrangements(line.0.as_str(), line.2.as_slice()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let p = parser!(lines(
        // char_of(".#?")+ string(" ") repeat_sep(i32, ",")
        string(any_char+) string(" ") repeat_sep(i32, ",")
    ));

    let v = p.parse(input).unwrap();

    Some(
        v.iter()
            .map(|line| {
                let mut s = String::new();
                let mut info = Vec::new();

                for i in 0..5 {
                    s += &line.0;
                    for e in &line.2 {
                        info.push(*e);
                    }
                    if i < 4 {
                        s += "?";
                    }
                }

                println!("Checking {} {:?}", s, info);
                find_arrangements(s.as_str(), &info)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(find_arrangements("???.###", &[1, 1, 3]), 1);
        assert_eq!(find_arrangements(".??..??...?##.", &[1, 1, 3]), 4);
        assert_eq!(find_arrangements("?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
        assert_eq!(find_arrangements("????.#...#...", &[4, 1, 1]), 1);
        assert_eq!(find_arrangements("????.######..#####.", &[1, 6, 5]), 4);
        assert_eq!(find_arrangements("?###????????", &[3, 2, 1]), 10);

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
