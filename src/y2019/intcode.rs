type ProgramArray = [u32; 1000];

pub struct Program {
	pub rom: ProgramArray,
}

#[derive(Clone)]
pub struct Interpreter {
	pub ram: ProgramArray,
	pc: usize,
}

impl From<&str> for Program {
	fn from(value: &str) -> Self {
		let mut rom = [0; 1000];
		for (i, op) in value.split(',').map(|opcode| opcode.parse().unwrap()).enumerate() {
			rom[i] = op;
		}
		Self { rom }
	}
}

impl From<&Program> for Interpreter {
	fn from(value: &Program) -> Self {
		Self { ram: value.rom, pc: 0 }
	}
}

impl Interpreter {
	fn read(&mut self) -> u32 {
		let value = self.ram[self.pc];
		self.pc += 1;
		value
	}

	pub fn run(&mut self) {
		loop {
			let opcode = self.read();
			if opcode == 99 {
				break;
			}
			self.exec(opcode);
		}
	}

	fn exec(&mut self, opcode: u32) {
		match opcode {
			1 => self.addition(),
			2 => self.multiplication(),
			_ => unimplemented!(),
		}
	}

	fn addition(&mut self) {
		let a = self.read() as usize;
		let b = self.read() as usize;
		let c = self.read() as usize;

		self.ram[c] = self.ram[a] + self.ram[b];
	}

	fn multiplication(&mut self) {
		let a = self.read() as usize;
		let b = self.read() as usize;
		let c = self.read() as usize;

		self.ram[c] = self.ram[a] * self.ram[b];
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn check_vec(left: &[u32], right: &[u32]) {
		for (l, r) in left.iter().zip(right.iter()) {
			assert_eq!(l, r);
		}
	}

	#[test]
	fn test_addition() {
		let program = Program::from("1,0,0,0,99");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_vec(&interpreter.ram, &[2, 0, 0, 0, 99]);
	}

	#[test]
	fn test_multiplication() {
		let program = Program::from("2,3,0,3,99");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_vec(&interpreter.ram, &[2, 3, 0, 6, 99]);

		let program = Program::from("2,4,4,5,99,0");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_vec(&interpreter.ram, &[2, 4, 4, 5, 99, 9801]);
	}

	#[test]
	fn test_addition_and_multiplication() {
		let program = Program::from("1,1,1,4,99,5,6,0,99");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_vec(&interpreter.ram, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);

		let program = Program::from("1,9,10,3,2,3,11,0,99,30,40,50");
		let mut interpreter = Interpreter::from(&program);
		interpreter.run();
		check_vec(&interpreter.ram, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}
}
