use crate::y2019::intcode::{Interpreter, Program};

pub fn parse_input(input: &str) -> Program {
	Program::from(input)
}

pub fn part1(input: &Program) -> u32 {
	let mut interpreter = Interpreter::from(input);
	interpreter.ram[1] = 12;
	interpreter.ram[2] = 2;
	interpreter.run();
	interpreter.ram[0]
}

pub fn part2(input: &Program) -> u32 {
	for noun in 0..100 {
		for verb in 0..100 {
			let mut interpreter = Interpreter::from(input);
			interpreter.ram[1] = noun;
			interpreter.ram[2] = verb;
			interpreter.run();
			if interpreter.ram[0] == 19690720 {
				return 100 * noun + verb;
			}
		}
	}
	unreachable!()
}
