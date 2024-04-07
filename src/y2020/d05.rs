use std::ops::RangeInclusive;

#[derive(Clone, Copy)]
enum Direction {
	Front,
	Back,
	Left,
	Right,
}

impl From<char> for Direction {
	fn from(value: char) -> Self {
		match value {
			'F' => Self::Front,
			'B' => Self::Back,
			'L' => Self::Left,
			'R' => Self::Right,
			_ => unimplemented!(),
		}
	}
}

pub struct BoardingPass {
	seat: [Direction; 10],
}

impl From<&str> for BoardingPass {
	fn from(value: &str) -> Self {
		let mut seat = [Direction::Back; 10];
		for (i, c) in value.chars().take(10).enumerate() {
			seat[i] = Direction::from(c)
		}
		Self { seat }
	}
}

fn find_position(range: RangeInclusive<u32>, directions: &[Direction]) -> u32 {
	let mut start = *range.start();
	let mut end = *range.end();

	for dir in directions {
		match dir {
			Direction::Front | Direction::Left => end = start + (end - start) / 2,
			Direction::Back | Direction::Right => start = end - (end - start) / 2,
		}
	}

	start
}

impl BoardingPass {
	fn row(&self) -> u32 {
		find_position(0..=127, &self.seat[..7])
	}

	fn column(&self) -> u32 {
		find_position(0..=7, &self.seat[7..])
	}

	fn id(&self) -> u32 {
		self.row() * 8 + self.column()
	}
}

pub fn parse_input(input: &str) -> Vec<BoardingPass> {
	input.lines().map(BoardingPass::from).collect()
}

pub fn part1(input: &[BoardingPass]) -> u32 {
	input.iter().map(|p| p.id()).max().unwrap_or(0)
}

pub fn part2(input: &[BoardingPass]) -> u32 {
	let mut ids = input.iter().map(|p| p.id()).collect::<Vec<_>>();
	ids.sort();
	for w in ids.windows(2) {
		if w[0] + 2 == w[1] {
			return w[0] + 1;
		}
	}
	unreachable!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		let example = BoardingPass::from("FBFBBFFRLR");
		assert_eq!(example.row(), 44);
		assert_eq!(example.column(), 5);
		assert_eq!(example.id(), 367);
	}
}
