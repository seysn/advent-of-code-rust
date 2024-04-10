use std::sync::mpsc::{channel, Receiver, Sender};

const MAX_PROGRAM_SIZE: usize = 5000;
const MAX_PARAMETERS: usize = 5;

pub struct Program {
	pub rom: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParameterMode {
	Position,
	Immediate,
	Relative,
}

impl From<i64> for ParameterMode {
	fn from(value: i64) -> Self {
		match value {
			1 => Self::Immediate,
			2 => Self::Relative,
			_ => Self::Position,
		}
	}
}

struct Instruction {
	opcode: i64,
	parameters: [ParameterMode; MAX_PARAMETERS],
}

impl From<i64> for Instruction {
	fn from(value: i64) -> Self {
		let opcode = value % 100;
		let mut parameters = [ParameterMode::Position; 5];
		let mut rest = value / 100;
		for param in parameters.iter_mut().take(MAX_PARAMETERS) {
			*param = ParameterMode::from(rest % 10);
			rest /= 10;
		}
		Self { opcode, parameters }
	}
}

pub struct Interpreter {
	pub ram: [i64; MAX_PROGRAM_SIZE],
	pc: usize,
	relative_base: i64,

	inputs: Option<Receiver<i64>>,
	outputs: Option<Sender<i64>>,

	pub last_output: Option<i64>,
}

impl From<&str> for Program {
	fn from(value: &str) -> Self {
		Self {
			rom: value.split(',').map(|opcode| opcode.parse().unwrap()).collect(),
		}
	}
}

impl From<&Program> for Interpreter {
	fn from(value: &Program) -> Self {
		let mut ram = [0; MAX_PROGRAM_SIZE];
		for (i, op) in value.rom.iter().enumerate() {
			ram[i] = *op;
		}

		Self {
			ram,
			pc: 0,
			relative_base: 0,
			inputs: None,
			outputs: None,
			last_output: None,
		}
	}
}

impl Interpreter {
	pub fn new(prog: &Program, inputs: Option<Receiver<i64>>, outputs: Option<Sender<i64>>) -> Self {
		let mut ram = [0; MAX_PROGRAM_SIZE];
		for (i, op) in prog.rom.iter().enumerate() {
			ram[i] = *op;
		}

		Self {
			ram,
			pc: 0,
			relative_base: 0,
			inputs,
			outputs,
			last_output: None,
		}
	}

	pub fn run_with_inputs(program: &Program, inputs: &[i64]) -> Vec<i64> {
		let out = {
			let (inputs_sender, inputs_recv) = channel();
			let (outputs_sender, outputs_recv) = channel();
			for inp in inputs {
				inputs_sender.send(*inp).unwrap();
			}
			let mut interpreter = Interpreter::new(program, Some(inputs_recv), Some(outputs_sender));
			interpreter.run();
			outputs_recv
		};
		out.iter().collect::<Vec<_>>()
	}

	pub fn run(&mut self) {
		loop {
			let instruction = Instruction::from(self.read());
			if instruction.opcode == 99 {
				break;
			}
			self.exec(instruction);
		}
	}

	fn read(&mut self) -> i64 {
		let value = self.ram[self.pc];
		self.pc += 1;
		value
	}

	fn read_index(&mut self, mode: &ParameterMode) -> usize {
		(match mode {
			ParameterMode::Position | ParameterMode::Immediate => self.read(),
			ParameterMode::Relative => self.relative_base + self.read(),
		}) as usize
	}

	fn read_parameter(&mut self, mode: &ParameterMode) -> i64 {
		match mode {
			ParameterMode::Position => self.ram[self.read() as usize],
			ParameterMode::Relative => self.ram[(self.relative_base + self.read()) as usize],
			ParameterMode::Immediate => self.read(),
		}
	}

	fn exec(&mut self, instruction: Instruction) {
		match instruction.opcode {
			0 => (),
			1 => self.addition(&instruction.parameters),
			2 => self.multiplication(&instruction.parameters),
			3 => self.pop_input(&instruction.parameters),
			4 => self.push_output(&instruction.parameters),
			5 => self.jump_if_true(&instruction.parameters),
			6 => self.jump_if_false(&instruction.parameters),
			7 => self.less_than(&instruction.parameters),
			8 => self.equals(&instruction.parameters),
			9 => self.set_relative_base(&instruction.parameters),
			_ => unimplemented!("opcode {}", instruction.opcode),
		}
	}

	fn addition(&mut self, parameters: &[ParameterMode]) {
		let a = self.read_parameter(&parameters[0]);
		let b = self.read_parameter(&parameters[1]);
		let idx = self.read_index(&parameters[2]);

		self.ram[idx] = a + b;
	}

	fn multiplication(&mut self, parameters: &[ParameterMode]) {
		let a = self.read_parameter(&parameters[0]);
		let b = self.read_parameter(&parameters[1]);
		let idx = self.read_index(&parameters[2]);

		self.ram[idx] = a * b;
	}

	fn pop_input(&mut self, parameters: &[ParameterMode]) {
		let idx = self.read_index(&parameters[0]);
		if let Some(r) = &self.inputs {
			if let Ok(i) = r.recv() {
				self.ram[idx] = i;
			}
		}
	}

	fn push_output(&mut self, parameters: &[ParameterMode]) {
		let value = self.read_parameter(&parameters[0]);

		if let Some(out) = &self.outputs {
			let _ = out.send(value);
			self.last_output = Some(value);
		}
	}

	fn jump_if_true(&mut self, parameters: &[ParameterMode]) {
		let a = self.read_parameter(&parameters[0]);
		let idx = self.read_parameter(&parameters[1]) as usize;
		if a != 0 {
			self.pc = idx;
		}
	}

	fn jump_if_false(&mut self, parameters: &[ParameterMode]) {
		let a = self.read_parameter(&parameters[0]);
		let idx = self.read_parameter(&parameters[1]) as usize;
		if a == 0 {
			self.pc = idx;
		}
	}

	fn less_than(&mut self, parameters: &[ParameterMode]) {
		let a = self.read_parameter(&parameters[0]);
		let b = self.read_parameter(&parameters[1]);
		let idx = self.read_index(&parameters[2]);
		self.ram[idx] = if a < b { 1 } else { 0 };
	}

	fn equals(&mut self, parameters: &[ParameterMode]) {
		let a = self.read_parameter(&parameters[0]);
		let b = self.read_parameter(&parameters[1]);
		let idx = self.read_index(&parameters[2]);
		self.ram[idx] = if a == b { 1 } else { 0 };
	}

	fn set_relative_base(&mut self, parameters: &[ParameterMode]) {
		let a = self.read_parameter(&parameters[0]);
		self.relative_base += a;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn check_slice(left: &[i64], right: &[i64]) {
		for (l, r) in left.iter().zip(right.iter()) {
			assert_eq!(l, r);
		}
	}

	#[test]
	fn test_addition() {
		let program = Program::from("1,0,0,0,99");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_slice(&interpreter.ram, &[2, 0, 0, 0, 99]);
	}

	#[test]
	fn test_multiplication() {
		let program = Program::from("2,3,0,3,99");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_slice(&interpreter.ram, &[2, 3, 0, 6, 99]);

		let program = Program::from("2,4,4,5,99,0");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_slice(&interpreter.ram, &[2, 4, 4, 5, 99, 9801]);
	}

	#[test]
	fn test_addition_and_multiplication() {
		let program = Program::from("1,1,1,4,99,5,6,0,99");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_slice(&interpreter.ram, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);

		let program = Program::from("1,9,10,3,2,3,11,0,99,30,40,50");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_slice(&interpreter.ram, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}

	#[test]
	fn test_instruction() {
		let inst = Instruction::from(1002);
		assert_eq!(inst.opcode, 2);
		assert_eq!(inst.parameters[0], ParameterMode::Position);
		assert_eq!(inst.parameters[1], ParameterMode::Immediate);
		assert_eq!(inst.parameters[2], ParameterMode::Position);

		let program = Program::from("1002,4,3,4,33");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_slice(&interpreter.ram, &[1002, 4, 3, 4, 99]);
	}

	#[test]
	fn test_negative_int() {
		let program = Program::from("1101,100,-1,4,0");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_slice(&interpreter.ram, &[1101, 100, -1, 4, 99])
	}

	#[test]
	fn test_input_output() {
		let program = Program::from("3,0,4,0,99");
		check_slice(&Interpreter::run_with_inputs(&program, &[42]), &[42]);

		let program = Program::from("4,0,99");
		check_slice(&Interpreter::run_with_inputs(&program, &[]), &[4]);

		let program = Program::from("3,0,1,0,6,0,4,0,99");
		check_slice(&Interpreter::run_with_inputs(&program, &[42]), &[46]);
	}

	#[test]
	fn test_equals() {
		let program = Program::from("3,9,8,9,10,9,4,9,99,-1,8");
		check_slice(&Interpreter::run_with_inputs(&program, &[8]), &[1]);

		let program = Program::from("3,9,8,9,10,9,4,9,99,-1,8");
		check_slice(&Interpreter::run_with_inputs(&program, &[4]), &[0]);

		let program = Program::from("3,3,1108,-1,8,3,4,3,99");
		check_slice(&Interpreter::run_with_inputs(&program, &[8]), &[1]);

		let program = Program::from("3,3,1108,-1,8,3,4,3,99");
		check_slice(&Interpreter::run_with_inputs(&program, &[4]), &[0]);
	}

	#[test]
	fn test_less_than() {
		let program = Program::from("3,9,7,9,10,9,4,9,99,-1,8");
		check_slice(&Interpreter::run_with_inputs(&program, &[7]), &[1]);

		let program = Program::from("3,9,7,9,10,9,4,9,99,-1,8");
		check_slice(&Interpreter::run_with_inputs(&program, &[9]), &[0]);

		let program = Program::from("3,3,1107,-1,8,3,4,3,99");
		check_slice(&Interpreter::run_with_inputs(&program, &[7]), &[1]);

		let program = Program::from("3,3,1107,-1,8,3,4,3,99");
		check_slice(&Interpreter::run_with_inputs(&program, &[9]), &[0]);
	}

	#[test]
	fn test_relative_base() {
		let program = Program::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
		check_slice(&Interpreter::run_with_inputs(&program, &[]), &program.rom);
		let program = Program::from("1102,34915192,34915192,7,4,7,99,0");
		assert!(Interpreter::run_with_inputs(&program, &[])[0] >= 1000000000000000);
		let program = Program::from("104,1125899906842624,99");
		assert_eq!(Interpreter::run_with_inputs(&program, &[])[0], 1125899906842624);

		let program = Program::from("109,-1,4,1,99");
		assert_eq!(Interpreter::run_with_inputs(&program, &[])[0], -1);
		let program = Program::from("109,-1,104,1,99");
		assert_eq!(Interpreter::run_with_inputs(&program, &[])[0], 1);
		let program = Program::from("109,-1,204,1,99");
		assert_eq!(Interpreter::run_with_inputs(&program, &[])[0], 109);
		let program = Program::from("109,1,9,2,204,-6,99");
		assert_eq!(Interpreter::run_with_inputs(&program, &[])[0], 204);
		let program = Program::from("109,1,109,9,204,-6,99");
		assert_eq!(Interpreter::run_with_inputs(&program, &[])[0], 204);
		let program = Program::from("109,1,209,-1,204,-106,99");
		assert_eq!(Interpreter::run_with_inputs(&program, &[])[0], 204);
		let program = Program::from("109,1,3,3,204,2,99");
		assert_eq!(Interpreter::run_with_inputs(&program, &[56])[0], 56);
		let program = Program::from("109,1,203,2,204,2,99");
		assert_eq!(Interpreter::run_with_inputs(&program, &[65])[0], 65);
	}
}
