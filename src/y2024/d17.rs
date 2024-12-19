use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Computer {
	a: u64,
	b: u64,
	c: u64,
	program: Vec<u64>,
}

pub fn parse_input(input: &str) -> Computer {
	let re = Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.+)").unwrap();
	let caps = re.captures(input).unwrap();

	Computer {
		a: caps[1].parse().unwrap(),
		b: caps[2].parse().unwrap(),
		c: caps[3].parse().unwrap(),
		program: caps[4].split(',').map(|i| i.parse().unwrap()).collect(),
	}
}

impl Computer {
	fn combo(&self, value: u64) -> u64 {
		match value {
			0..=3 => value,
			4 => self.a,
			5 => self.b,
			6 => self.c,
			_ => unreachable!(),
		}
	}

	fn run(&mut self) -> Vec<u64> {
		let mut pointer = 0;
		let mut output = Vec::new();
		while pointer < self.program.len() {
			let opcode = self.program[pointer];
			let arg = self.program[pointer + 1];

			match opcode {
				0 => {
					let numerator = self.a;
					let denominator = 2u64.pow(self.combo(arg) as u32);
					self.a = numerator / denominator;
				}
				1 => {
					self.b ^= arg;
				}
				2 => {
					self.b = self.combo(arg).rem_euclid(8);
				}
				3 => {
					if self.a != 0 {
						pointer = arg as usize;
						continue;
					}
				}
				4 => {
					self.b ^= self.c;
				}
				5 => {
					output.push(self.combo(arg).rem_euclid(8));
				}
				6 => {
					let numerator = self.a;
					let denominator = 2u64.pow(self.combo(arg) as u32);
					self.b = numerator / denominator;
				}
				7 => {
					let numerator = self.a;
					let denominator = 2u64.pow(self.combo(arg) as u32);
					self.c = numerator / denominator;
				}
				_ => unreachable!(),
			}

			pointer += 2;
		}

		output
	}
}

pub fn part1(input: &Computer) -> String {
	let mut computer = input.clone();
	let output = computer.run();

	output.iter().join(",")
}

/// I don't know if it works for other input, but for me the logic was:
/// the last 3 bits of A changes the last value of output
/// the previous 3 bits of A changes the previous value of output
/// and so on...
/// so we're trying every combinations possible and return the minimum one at the end
pub fn part2(input: &Computer) -> u64 {
	let mut computer = input.clone();
	let mut candidates = vec![0];

	for j in 0..input.program.len() {
		let mut new = Vec::new();

		for candidate in candidates {
			for i in 0..8 {
				let aa = (candidate << 3) + i;
				computer.a = aa;
				let output = computer.run();

				let idx_program = input.program.len() - j - 1;
				let idx_output = output.len() - j - 1;
				if input.program[idx_program] == output[idx_output] {
					new.push(aa);
				}
			}
		}

		candidates = new;
	}

	*candidates.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

	#[test]
	fn example_part1() {
		let mut c = Computer {
			a: 0,
			b: 0,
			c: 9,
			program: vec![2, 6],
		};
		c.run();
		assert_eq!(c.b, 1);

		let mut c = Computer {
			a: 10,
			b: 0,
			c: 0,
			program: vec![5, 0, 5, 1, 5, 4],
		};
		let out = c.run();
		assert_eq!(&out, &[0, 1, 2]);

		let mut c = Computer {
			a: 2024,
			b: 0,
			c: 0,
			program: vec![0, 1, 5, 4, 3, 0],
		};
		let out = c.run();
		assert_eq!(&out, &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
		assert_eq!(c.a, 0);

		let mut c = Computer {
			a: 0,
			b: 29,
			c: 0,
			program: vec![1, 7],
		};
		c.run();
		assert_eq!(c.b, 26);

		let mut c = Computer {
			a: 0,
			b: 2024,
			c: 43690,
			program: vec![4, 0],
		};
		c.run();
		assert_eq!(c.b, 44354);

		assert_eq!(part1(&parse_input(EXAMPLE)), "4,6,3,5,6,3,5,2,1,0".to_owned());
	}
}
