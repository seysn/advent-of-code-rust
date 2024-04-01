use std::{sync::mpsc::channel, thread};

use itertools::Itertools;

use super::intcode::{Interpreter, Program};

pub fn parse_input(input: &str) -> Program {
	Program::from(input)
}

pub fn part1(input: &Program) -> i32 {
	let mut res = i32::MIN;
	for seq in (0..5).permutations(5) {
		let mut out = 0;
		for phase in seq {
			out = Interpreter::run_with_inputs(input, &[phase, out])[0];
		}
		res = res.max(out);
	}
	res
}

fn run_amplifiers(input: &Program, seq: &[i32]) -> i32 {
	let (outputs_sender_e, inputs_recv_a) = channel();
	let (outputs_sender_a, inputs_recv_b) = channel();
	let (outputs_sender_b, inputs_recv_c) = channel();
	let (outputs_sender_c, inputs_recv_d) = channel();
	let (outputs_sender_d, inputs_recv_e) = channel();
	outputs_sender_e.send(seq[0]).unwrap();
	outputs_sender_e.send(0).unwrap();
	outputs_sender_a.send(seq[1]).unwrap();
	outputs_sender_b.send(seq[2]).unwrap();
	outputs_sender_c.send(seq[3]).unwrap();
	outputs_sender_d.send(seq[4]).unwrap();

	let mut amp_a = Interpreter::new(input, inputs_recv_a, outputs_sender_a);
	let mut amp_b = Interpreter::new(input, inputs_recv_b, outputs_sender_b);
	let mut amp_c = Interpreter::new(input, inputs_recv_c, outputs_sender_c);
	let mut amp_d = Interpreter::new(input, inputs_recv_d, outputs_sender_d);
	let mut amp_e = Interpreter::new(input, inputs_recv_e, outputs_sender_e);

	thread::spawn(move || amp_a.run());
	thread::spawn(move || amp_b.run());
	thread::spawn(move || amp_c.run());
	thread::spawn(move || amp_d.run());
	let e = thread::spawn(move || {
		amp_e.run();
		amp_e.last_output
	});

	e.join().unwrap().unwrap()
}

pub fn part2(input: &Program) -> i32 {
	let mut res = i32::MIN;

	for seq in (5..10).permutations(5) {
		res = res.max(run_amplifiers(input, &seq));
	}
	res
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")), 43210);
		assert_eq!(
			part1(&parse_input(
				"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
			)),
			54321
		);
		assert_eq!(
			part1(&parse_input(
				"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
			)),
			65210
		);
	}

	#[test]
	fn example_part2() {
		assert_eq!(
			part2(&parse_input(
				"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
			)),
			139629729
		);
	}
}
