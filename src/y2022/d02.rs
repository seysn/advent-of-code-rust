#[derive(Clone, Copy)]
pub enum Shape {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl Shape {
	fn from_char(c: char) -> Self {
		match c {
			'A' | 'X' => Shape::Rock,
			'B' | 'Y' => Shape::Paper,
			'C' | 'Z' => Shape::Scissors,
			_ => unreachable!(),
		}
	}
}

enum Win {
	Me,
	Noob,
	Draw,
}

#[derive(Debug)]
pub struct Round {
	me: Shape,
	noob: Shape,
}

impl Round {
	fn win(&self) -> Win {
		match self.me {
			Shape::Paper => match self.noob {
				Shape::Rock => Win::Me,
				Shape::Paper => Win::Draw,
				Shape::Scissors => Win::Noob,
			},
			Shape::Rock => match self.noob {
				Shape::Rock => Win::Draw,
				Shape::Paper => Win::Noob,
				Shape::Scissors => Win::Me,
			},
			Shape::Scissors => match self.noob {
				Shape::Rock => Win::Noob,
				Shape::Paper => Win::Me,
				Shape::Scissors => Win::Draw,
			}
		}
	}

	fn choice(&self) -> Shape {
		match self.noob {
			Shape::Paper => match self.me {
				Shape::Rock => Shape::Rock,
				Shape::Paper => Shape::Paper,
				Shape::Scissors => Shape::Scissors,
			},
			Shape::Rock => match self.me {
				Shape::Rock => Shape::Scissors,
				Shape::Paper => Shape::Rock,
				Shape::Scissors => Shape::Paper,
			},
			Shape::Scissors => match self.me {
				Shape::Rock => Shape::Paper,
				Shape::Paper => Shape::Scissors,
				Shape::Scissors => Shape::Rock,
			}
		}
	}
}

pub fn parse_input(input: &str) -> Vec<Round> {
	input.lines().map(|l| Round {
		me: Shape::from_char(l.chars().nth(2).unwrap()),
		noob: Shape::from_char(l.chars().next().unwrap()),
	}).collect()
}

pub fn part1(input: &[Round]) -> i32 {
	let mut points = 0;
	for round in input {
		points += round.me as i32;
		points += match round.win() {
			Win::Me => 6,
			Win::Noob => 0,
			Win::Draw => 3,
		};
	}
	points
}

pub fn part2(input: &[Round]) -> i32 {
	let mut points = 0;
	for round in input {
		points += round.choice() as i32;
		points += match round.me {
			Shape::Rock => 0,
			Shape::Paper => 3,
			Shape::Scissors => 6,
		};
	}
	points
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "A Y
B X
C Z";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 15);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 12);
	}
}
