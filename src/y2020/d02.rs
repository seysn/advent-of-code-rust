use regex::Regex;

pub struct Password {
    start: usize,
    end: usize,
    letter: char,
    password: String,
}

// #[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let re = Regex::new(r"(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)").unwrap();
            let cap = re.captures(l).unwrap();
            Password {
                start: cap[1].parse().unwrap(),
                end: cap[2].parse().unwrap(),
                letter: cap[3].chars().nth(0).unwrap(),
                password: cap[4].to_string(),
            }
        })
        .collect()
}

// #[aoc(day2, part1)]
pub fn part1(input: &[Password]) -> u32 {
    let mut res = 0;
    for p in input {
        let count = p.password.chars().filter(|&c| c == p.letter).count();
        if p.start <= count && count <= p.end {
            res += 1;
        }
    }

    res
}

// #[aoc(day2, part2)]
pub fn part2(input: &[Password]) -> u32 {
    let mut res = 0;
    for p in input {
        if (p.password.chars().nth(p.start - 1).unwrap() == p.letter)
            ^ (p.password.chars().nth(p.end - 1).unwrap() == p.letter)
        {
            res += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 1);
    }
}