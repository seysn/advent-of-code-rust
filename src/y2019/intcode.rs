#[derive(Clone)]
pub struct Interpreter {
	pub ram: [u32; 1000],
	pc: usize,
}

impl From<&str> for Interpreter {
	fn from(value: &str) -> Self {
		let mut ram = [0; 1000];
		for (i, op) in value.split(',').map(|opcode| opcode.parse().unwrap()).enumerate() {
			ram[i] = op;
		}
		Self { ram, pc: 0 }
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
