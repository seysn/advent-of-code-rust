use super::intcode::{Interpreter, Program};

pub fn parse_input(input: &str) -> Program {
	Program::from(input)
}

pub fn part1(input: &Program) -> i64 {
	Interpreter::run_with_inputs(input, &[1])[0]
}

pub fn part2(input: &Program) -> i64 {
	Interpreter::run_with_inputs(input, &[2])[0]
}
