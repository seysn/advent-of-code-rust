use super::intcode::{Interpreter, Program};

pub fn parse_input(input: &str) -> Program {
	Program::from(input)
}

pub fn part1(input: &Program) -> i32 {
	Interpreter::from(input).run_with_inputs(&[1])[0]
}

pub fn part2(input: &Program) -> i32 {
	Interpreter::from(input).run_with_inputs(&[5])[0]
}
