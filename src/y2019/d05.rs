use super::intcode::{Interpreter, Program};

pub fn parse_input(input: &str) -> Program {
	Program::from(input)
}

pub fn part1(input: &Program) -> i32 {
	*Interpreter::run_with_inputs(input, &[1]).iter().last().unwrap()
}

pub fn part2(input: &Program) -> i32 {
	Interpreter::run_with_inputs(input, &[5])[0]
}
