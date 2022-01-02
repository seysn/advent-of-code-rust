pub fn parse_input(content: &str) -> Vec<(String, i32)> {
    content.lines().map(|l| {
        let mut t = l.split(" ");
        (t.next().unwrap().to_owned(), t.next().unwrap().parse().unwrap())
    }).collect()
}

pub fn part1(input: &[(String, i32)]) -> i32 {
    let mut depth = 0;
    let mut pos = 0;

    for (inp, i) in input.iter() {
        match inp.as_ref() {
            "forward" => pos += i,
            "up" => depth -= i,
            "down" => depth += i,
            _ => panic!("no"),
        }
    }

    depth * pos
}

pub fn part2(input: &[(String, i32)]) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;

    for (inp, i) in input.iter() {
        match inp.as_str() {
            "forward" => {
                pos += i;
                depth += aim * i;
            },
            "up" => aim -= i,
            "down" => aim += i,
            _ => panic!("no"),
        }
    }

    depth * pos
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 150);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 900);
    }
}