pub struct Line {
    chars: Vec<char>
}

impl Line {
    pub fn corrupted_value(&self) -> u32 {
        let mut stack: Vec<char> = Vec::new();
        for &c in &self.chars {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        return 3;
                    }
                },
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        return 57;
                    }
                },
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        return 1197;
                    }
                },
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        return 25137;
                    }
                },
                _ => unreachable!()
            }
        }

        0
    }

    pub fn incomplete_value(&self) -> u64 {
        let mut stack: Vec<char> = Vec::new();
        for &c in &self.chars {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        return 0;
                    }
                },
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        return 0;
                    }
                },
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        return 0;
                    }
                },
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        return 0;
                    }
                },
                _ => unreachable!()
            }
        }

        stack.iter().rev().fold(0, |cpt, &c| cpt * 5 + char_points(c))
    }
}

fn char_points(c: char) -> u64 {
    match c {
        '(' | ')' => 1,
        '[' | ']' => 2,
        '{' | '}' => 3,
        '<' | '>' => 4,
        _ => unreachable!()
    }
}

pub fn parse_input(content: &str) -> Vec<Line> {
    content.lines().map(|l| Line {chars: l.chars().collect()}).collect()
}

pub fn part1(input: &[Line]) -> u32 {
    input.iter().map(|l| l.corrupted_value()).sum()
}

pub fn part2(input: &[Line]) -> u64 {
    let mut points: Vec<u64> = input.iter().map(|l| l.incomplete_value()).filter(|&x| x != 0).collect();
    points.sort();

    points[points.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 26397);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 288957);
    }
}