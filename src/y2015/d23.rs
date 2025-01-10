#[derive(Debug)]
pub enum Register {
	A,
	B,
}

#[derive(Debug)]
pub enum Instruction {
	Half(Register),
	Triple(Register),
	Increment(Register),
	Jump(isize),
	JumpIfEven(Register, isize),
	JumpIfOne(Register, isize),
}

impl Register {
	fn new(c: char) -> Self {
		match c {
			'a' => Self::A,
			'b' => Self::B,
			_ => unimplemented!("{c}"),
		}
	}
}

impl Instruction {
	fn new(line: &str) -> Self {
		match &line[..3] {
			"hlf" => Self::Half(Register::new(line.chars().nth(4).unwrap())),
			"tpl" => Self::Triple(Register::new(line.chars().nth(4).unwrap())),
			"inc" => Self::Increment(Register::new(line.chars().nth(4).unwrap())),
			"jmp" => Self::Jump(line[4..].parse().unwrap()),
			"jie" => Self::JumpIfEven(Register::new(line.chars().nth(4).unwrap()), line[7..].parse().unwrap()),
			"jio" => Self::JumpIfOne(Register::new(line.chars().nth(4).unwrap()), line[7..].parse().unwrap()),
			_ => unimplemented!("{line}"),
		}
	}
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	input.lines().map(Instruction::new).collect()
}

#[derive(Default)]
struct Registers {
	a: i32,
	b: i32,
}

impl Registers {
	fn get(&self, reg: &Register) -> i32 {
		match reg {
			Register::A => self.a,
			Register::B => self.b,
		}
	}

	fn set(&mut self, reg: &Register, value: i32) {
		match reg {
			Register::A => self.a = value,
			Register::B => self.b = value,
		}
	}
}

fn run(input: &[Instruction], mut regs: Registers) -> Registers {
	let mut pc: isize = 0;
	while pc < input.len() as isize {
		let mut jumped = false;
		let instruction = &input[pc as usize];
		match instruction {
			Instruction::Half(r) => regs.set(r, regs.get(r) / 2),
			Instruction::Triple(r) => regs.set(r, regs.get(r) * 3),
			Instruction::Increment(r) => regs.set(r, regs.get(r) + 1),
			Instruction::Jump(i) => {
				pc += i;
				jumped = true
			}
			Instruction::JumpIfEven(r, i) => {
				if regs.get(r) % 2 == 0 {
					pc += i;
					jumped = true;
				}
			}
			Instruction::JumpIfOne(r, i) => {
				if regs.get(r) == 1 {
					pc += i;
					jumped = true;
				}
			}
		}

		if !jumped {
			pc += 1;
		}
	}

	regs
}

pub fn part1(input: &[Instruction]) -> i32 {
	run(input, Registers::default()).b
}

pub fn part2(input: &[Instruction]) -> i32 {
	run(input, Registers { a: 1, b: 0 }).b
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "inc a
jio a, +2
tpl a
inc a";

	#[test]
	fn example_part1() {
		assert_eq!(run(&parse_input(EXAMPLE), Registers::default()).a, 2);
	}
}
