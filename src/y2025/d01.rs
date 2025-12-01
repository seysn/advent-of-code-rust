pub enum Direction {
	Left,
	Right,
}

pub struct Instruction {
	direction: Direction,
	step: u32,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|l| {
			let mut chars = l.chars();
			let direction = match chars.next().unwrap() {
				'L' => Direction::Left,
				'R' => Direction::Right,
				_ => unreachable!(),
			};

			Instruction {
				direction,
				step: l[1..].parse().unwrap(),
			}
		})
		.collect()
}

pub fn part1(input: &[Instruction]) -> usize {
	let mut res = 0;
	let mut dial = 50;
	for Instruction { direction, step } in input {
		let v = match direction {
			Direction::Left => -((*step) as i32),
			Direction::Right => *step as i32,
		};

		dial += v;

		while dial < 0 {
			dial += 100;
		}

		while dial >= 100 {
			dial -= 100;
		}

		if dial == 0 {
			res += 1;
		}
	}

	res
}

pub fn part2(input: &[Instruction]) -> i32 {
	let mut res = 0;
	let mut dial = 50;
	for Instruction { direction, step } in input {
		for _ in 0..*step {
			match direction {
				Direction::Left => dial -= 1,
				Direction::Right => dial += 1,
			}

			dial %= 100;
			if dial == 0 {
				res += 1;
			}
		}
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 3);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 6);
		assert_eq!(part2(&parse_input("R1000")), 10);
	}
}
