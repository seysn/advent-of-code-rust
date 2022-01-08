pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn length(s: &String) -> (usize, usize) {
    let mut raw = 0;
    let mut data = 0;
    let mut iter = s.chars();

    loop {
        let c = iter.next();
        if c.is_none() {
            break;
        }

        let mut c = c.unwrap();
        raw += 1;
        data += 1;
        if c == '\\' {
            c = iter.next().unwrap();
            raw += 1;
            if c == 'x' {
                raw += 2;
                iter.next();
                iter.next();
            }
        }
    }

    data -= 2; // Removing quotes
    (raw, data)
}

fn encode(s: &String) -> String {
    let mut iter = s.chars();
    let mut res = String::new();
    res.push('"');

    loop {
        let c = iter.next();
        if c.is_none() {
            break;
        }

        let c = c.unwrap();
        match c {
            '"' => res.push_str("\\\""),
            '\\' => res.push_str("\\\\"),
            _ => res.push(c),
        }
    }

    res.push('"');
    res
}

pub fn part1(input: &[String]) -> usize {
    input.iter().fold(0, |acc, s| {
        let (raw, data) = length(s);
        acc + raw - data
    })
}

pub fn part2(input: &[String]) -> usize {
    input.iter().fold(0, |acc, s| {
        let (raw, _) = length(s);
        let (raw_encoded, _) = length(&encode(s));
        acc + raw_encoded - raw
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 12);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 19);
    }
}