use crate::y2019::intcode::Interpreter;

pub fn parse_input(input: &str) -> Interpreter {
	Interpreter::from(input)
}

pub fn part1(input: &Interpreter) -> u32 {
	let mut interpreter = input.clone();
	interpreter.ram[1] = 12;
	interpreter.ram[2] = 2;
	interpreter.run();
	interpreter.ram[0]
}

pub fn part2(input: &Interpreter) -> u32 {
	for noun in 0..100 {
		for verb in 0..100 {
			let mut interpreter = input.clone();
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		let mut interpreter = Interpreter::from("1,0,0,0,99");
		interpreter.run();
		for (i, &op) in [2, 0, 0, 0, 99].iter().enumerate() {
			assert_eq!(interpreter.ram[i], op);
		}

		let mut interpreter = Interpreter::from("2,3,0,3,99");
		interpreter.run();
		for (i, &op) in [2, 3, 0, 6, 99].iter().enumerate() {
			assert_eq!(interpreter.ram[i], op);
		}

		let mut interpreter = Interpreter::from("2,4,4,5,99,0");
		interpreter.run();
		for (i, &op) in [2, 4, 4, 5, 99, 9801].iter().enumerate() {
			assert_eq!(interpreter.ram[i], op);
		}

		let mut interpreter = Interpreter::from("1,1,1,4,99,5,6,0,99");
		interpreter.run();
		for (i, &op) in [30, 1, 1, 4, 2, 5, 6, 0, 99].iter().enumerate() {
			assert_eq!(interpreter.ram[i], op);
		}

		let mut interpreter = Interpreter::from("1,9,10,3,2,3,11,0,99,30,40,50");
		interpreter.run();
		for (i, &op) in [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50].iter().enumerate() {
			assert_eq!(interpreter.ram[i], op);
		}
	}
}
