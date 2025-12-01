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
			if let Some(step) = l.strip_prefix('L') {
				Instruction {
					direction: Direction::Left,
					step: step.parse().unwrap(),
				}
			} else if let Some(step) = l.strip_prefix('R') {
				Instruction {
					direction: Direction::Right,
					step: step.parse().unwrap(),
				}
			} else {
				panic!("Instruction '{l}' is not valid");
			}
		})
		.collect()
}

pub fn part1(input: &[Instruction]) -> usize {
	let mut res = 0;
	let mut dial = 50;
	for Instruction { direction, step } in input {
		let step = match direction {
			Direction::Left => -((*step) as i32),
			Direction::Right => *step as i32,
		};

		dial += step;
		dial %= 100;

		if dial == 0 {
			res += 1;
		}
	}

	res
}

pub fn part2(input: &[Instruction]) -> i32 {
	let mut res = 0;
	let mut dial = 50;
	for instruction in input {
		match instruction {
			Instruction {
				direction: Direction::Left,
				step,
			} => {
				let step = *step as i32;

				// First we count whole 100 turns
				res += step / 100;

				// Then we count cases where we go to negatives with the reminder of 100
				if dial != 0 && step % 100 >= dial {
					res += 1;
				}

				// Finally we can make the move
				dial = (dial - step).rem_euclid(100);
			}
			Instruction {
				direction: Direction::Right,
				step,
			} => {
				let step = *step as i32;

				// First we make the move
				dial += step;

				// Then we can just remove every 100s and put them in the counter
				res += dial / 100;
				dial %= 100;
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
