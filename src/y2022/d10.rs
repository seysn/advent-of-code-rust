#[derive(Debug)]
pub enum Instruction {
	Noop,
	Addx(i32),
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|l| {
			if let Some(v) = l.strip_prefix("addx ") {
				Instruction::Addx(v.parse().unwrap())
			} else {
				Instruction::Noop
			}
		})
		.collect()
}

fn generate_cycles(instructions: &[Instruction]) -> Vec<i32> {
	let mut cycles = Vec::new();

	for inst in instructions {
		match inst {
			Instruction::Noop => cycles.push(0),
			Instruction::Addx(v) => {
				cycles.push(0);
				cycles.push(*v);
			}
		}
	}

	cycles
}

pub fn part1(input: &[Instruction]) -> i32 {
	let mut res = 0;
	let mut x = 1;
	let cycles = generate_cycles(input);
	let mut iter = cycles.iter().cycle();

	x += iter.by_ref().take(19).sum::<i32>();
	res += x * 20;
	x += iter.next().unwrap();

	for i in 1..=5 {
		x += iter.by_ref().take(39).sum::<i32>();
		res += x * (i * 40 + 20);
		x += iter.next().unwrap();
	}

	res
}

pub fn part2(input: &[Instruction]) -> String {
	let mut res = String::new();
	let mut x = 1;
	let cycles = generate_cycles(input);
	let mut iter = cycles.iter().cycle();

	for i in 0..240 {
		if i % 40 == 0 {
			res.push('\n')
		}

		if x - 1 <= i % 40 && i % 40 <= x + 1 {
			res.push('█');
		} else {
			res.push(' ');
		}

		x += iter.next().unwrap();
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 13140);
	}

	#[test]
	fn example_part2() {
		const EXPECTED: &str = "
██  ██  ██  ██  ██  ██  ██  ██  ██  ██  
███   ███   ███   ███   ███   ███   ███ 
████    ████    ████    ████    ████    
█████     █████     █████     █████     
██████      ██████      ██████      ████
███████       ███████       ███████     ";

		assert_eq!(part2(&parse_input(EXAMPLE)), EXPECTED.to_string());
	}
}
