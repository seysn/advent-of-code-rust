#[derive(PartialEq, Eq)]
pub enum Instruction {
	Up,
	Down,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	input
		.chars()
		.map(|c| if c == '(' { Instruction::Up } else { Instruction::Down })
		.collect()
}

pub fn part1(input: &[Instruction]) -> i32 {
	input
		.iter()
		.fold(0, |acc, inst| if inst == &Instruction::Up { acc + 1 } else { acc - 1 })
}

pub fn part2(input: &[Instruction]) -> usize {
	let mut acc = 0;
	for (idx, inst) in input.iter().enumerate() {
		if inst == &Instruction::Up {
			acc += 1;
		} else {
			acc -= 1;
		}

		if acc == -1 {
			return idx + 1;
		}
	}

	unreachable!();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input("(())")), 0);
		assert_eq!(part1(&parse_input("()()")), 0);
		assert_eq!(part1(&parse_input("(((")), 3);
		assert_eq!(part1(&parse_input("(()(()(")), 3);
		assert_eq!(part1(&parse_input("))(((((")), 3);
		assert_eq!(part1(&parse_input("())")), -1);
		assert_eq!(part1(&parse_input("))(")), -1);
		assert_eq!(part1(&parse_input(")))")), -3);
		assert_eq!(part1(&parse_input(")())())")), -3);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(")")), 1);
		assert_eq!(part2(&parse_input("()())")), 5);
	}
}
