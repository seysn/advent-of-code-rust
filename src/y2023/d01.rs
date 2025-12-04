pub fn parse_input(input: &str) -> Vec<String> {
	input.lines().map(|l| l.to_string()).collect()
}

pub fn part1(input: &[String]) -> u32 {
	input
		.iter()
		.map(|l| {
			let s = l.chars().filter(|c| c.is_numeric()).collect::<String>();
			let fst = s.chars().next().unwrap().to_digit(10).unwrap();
			let lst = s.chars().last().unwrap().to_digit(10).unwrap();
			fst * 10 + lst
		})
		.sum()
}

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn str_to_digit(s: &str) -> u32 {
	match s {
		"one" => 1,
		"two" => 2,
		"three" => 3,
		"four" => 4,
		"five" => 5,
		"six" => 6,
		"seven" => 7,
		"eight" => 8,
		"nine" => 9,
		_ => unreachable!(),
	}
}

pub fn part2(input: &[String]) -> u32 {
	input
		.iter()
		.map(|l| {
			let mut res = Vec::new();
			for (idx, c) in l.char_indices() {
				if c.is_numeric() {
					res.push(c.to_digit(10).unwrap());
					continue;
				}

				for digit in DIGITS {
					if idx + digit.len() <= l.len() && &l[idx..idx + digit.len()] == digit {
						res.push(str_to_digit(digit));
						break;
					}
				}
			}

			let fst = res.first().unwrap();
			let lst = res.iter().last().unwrap();
			fst * 10 + lst
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

	const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 142);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE2)), 281);
	}
}
