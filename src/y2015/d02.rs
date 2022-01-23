pub struct Dimension {
	length: u32,
	width: u32,
	height: u32,
}

impl Dimension {
	fn parse(line: &str) -> Dimension {
		let d: Vec<u32> = line.split("x").map(|v| v.parse().unwrap()).collect();
		Dimension {
			length: d[0],
			width: d[1],
			height: d[2],
		}
	}

	fn surface(&self) -> u32 {
		let areas = [self.length * self.width, self.width * self.height, self.height * self.length];
		areas.iter().map(|area| area * 2).sum::<u32>() + areas.iter().min().unwrap()
	}

	fn ribbon(&self) -> u32 {
		let mut values = [self.length, self.width, self.height];
		values.sort();

		values[0] * 2 + values[1] * 2 + self.length * self.width * self.height
	}
}

pub fn parse_input(input: &str) -> Vec<Dimension> {
	input.lines().map(|l| Dimension::parse(l)).collect()
}

pub fn part1(input: &[Dimension]) -> u32 {
	input.iter().map(|dim| dim.surface()).sum()
}

pub fn part2(input: &[Dimension]) -> u32 {
	input.iter().map(|dim| dim.ribbon()).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE1: &'static str = "2x3x4";
	const EXAMPLE2: &'static str = "1x1x10";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE1)), 58);
		assert_eq!(part1(&parse_input(EXAMPLE2)), 43);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE1)), 34);
		assert_eq!(part2(&parse_input(EXAMPLE2)), 14);
	}
}
