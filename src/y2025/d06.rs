#[derive(Debug, Clone, Copy)]
pub enum Sign {
	Add,
	Mul,
}

impl From<&str> for Sign {
	fn from(s: &str) -> Self {
		match s {
			"+" => Self::Add,
			"*" => Self::Mul,
			_ => panic!("Sign '{s}' is not valid"),
		}
	}
}

pub fn parse_input(input: &str) -> Vec<String> {
	input.lines().map(|l| l.to_owned()).collect()
}

pub fn part1(input: &[String]) -> u64 {
	let mut numbers: Vec<Vec<u64>> = Vec::new();
	for line in &input[..input.len() - 1] {
		numbers.push(line.split_whitespace().map(|s| s.parse().unwrap()).collect());
	}

	let signs: Vec<Sign> = input.last().unwrap().split_whitespace().map(Sign::from).collect();

	let mut res = 0;
	for (i, sign) in signs.iter().enumerate() {
		match sign {
			Sign::Add => {
				let mut acc = 0;
				for numbers in &numbers {
					acc += numbers[i];
				}
				res += acc;
			}
			Sign::Mul => {
				let mut acc = 1;
				for numbers in &numbers {
					acc *= numbers[i];
				}
				res += acc;
			}
		}
	}

	res
}

pub fn part2(input: &[String]) -> u64 {
	let numbers: Vec<Vec<char>> = input[..input.len() - 1].iter().map(|l| l.chars().collect()).collect();
	let signs: Vec<Sign> = input.last().unwrap().split_whitespace().map(Sign::from).collect();

	let mut res = 0;
	let mut sign = 0;
	let mut acc = match signs[0] {
		Sign::Add => 0,
		Sign::Mul => 1,
	};

	for i in 0..numbers[0].len() {
		let mut number = 0;

		for n in &numbers {
			let ch = n[i];
			if ch.is_whitespace() {
				continue;
			}

			number *= 10;
			number += ch.to_digit(10).unwrap() as u64;
		}

		if number == 0 {
			res += acc;
			sign += 1;
			acc = match signs[sign] {
				Sign::Add => 0,
				Sign::Mul => 1,
			};
			continue;
		}

		match signs[sign] {
			Sign::Add => acc += number,
			Sign::Mul => acc *= number,
		}
	}

	// Adding the last one
	res += acc;

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 4277556);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 3263827);
	}
}
