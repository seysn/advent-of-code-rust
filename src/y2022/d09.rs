use std::{
	collections::HashSet,
	ops::{AddAssign, Sub},
};

#[derive(Clone, Copy)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn parse(c: &str) -> Direction {
		match c {
			"U" => Direction::Up,
			"D" => Direction::Down,
			"L" => Direction::Left,
			"R" => Direction::Right,
			_ => unreachable!(),
		}
	}

	fn delta(self) -> (i32, i32) {
		match self {
			Direction::Up => (0, 1),
			Direction::Down => (0, -1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
		}
	}
}

#[derive(Clone, Copy)]
pub struct Instruction {
	direction: Direction,
	distance: usize,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|l| {
			let (dir, dist) = l.split_once(' ').unwrap();
			Instruction {
				direction: Direction::parse(dir),
				distance: dist.parse().unwrap(),
			}
		})
		.collect()
}

#[derive(Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
	x: i32,
	y: i32,
}

impl AddAssign<(i32, i32)> for Position {
	fn add_assign(&mut self, delta: (i32, i32)) {
		*self = Self {
			x: self.x + delta.0,
			y: self.y + delta.1,
		};
	}
}

impl Sub<Position> for Position {
	type Output = (i32, i32);

	fn sub(self, other: Position) -> Self::Output {
		(self.x - other.x, self.y - other.y)
	}
}

#[derive(Default)]
struct Rope {
	head: Position,
	tail: Position,
}

impl AddAssign<Direction> for Rope {
	fn add_assign(&mut self, dir: Direction) {
		self.head += dir.delta();
		let delta = self.head - self.tail;
		self.tail += match delta {
			(2, 0) => (1, 0),
			(-2, 0) => (-1, 0),
			(0, 2) => (0, 1),
			(0, -2) => (0, -1),
			(1, 2) | (2, 1) => (1, 1),
			(-1, 2) | (-2, 1) => (-1, 1),
			(1, -2) | (2, -1) => (1, -1),
			(-1, -2) | (-2, -1) => (-1, -1),
			_ => (0, 0),
		}
	}
}

pub fn part1(input: &[Instruction]) -> usize {
	let mut rope = Rope::default();
	let mut positions: HashSet<Position> = HashSet::new();

	for inst in input {
		for _ in 0..inst.distance {
			rope += inst.direction;
			positions.insert(rope.tail);
		}
	}

	positions.len()
}

struct LongRope {
	knots: Vec<Position>,
}

impl AddAssign<Direction> for LongRope {
	fn add_assign(&mut self, dir: Direction) {
		self.knots[0] += dir.delta();
		for i in 0..self.knots.len() - 1 {
			let delta = self.knots[i] - self.knots[i + 1];
			self.knots[i + 1] += match delta {
				(2, 0) => (1, 0),
				(-2, 0) => (-1, 0),
				(0, 2) => (0, 1),
				(0, -2) => (0, -1),
				(1, 2) | (2, 1) => (1, 1),
				(-1, 2) | (-2, 1) => (-1, 1),
				(1, -2) | (2, -1) => (1, -1),
				(-1, -2) | (-2, -1) => (-1, -1),
				(2, 2) => (1, 1),
				(-2, 2) => (-1, 1),
				(2, -2) => (1, -1),
				(-2, -2) => (-1, -1),
				_ => (0, 0),
			}
		}
	}
}

pub fn part2(input: &[Instruction]) -> usize {
	let mut rope = LongRope {
		knots: vec![Position::default(); 10],
	};
	let mut positions: HashSet<Position> = HashSet::new();

	for inst in input {
		for _ in 0..inst.distance {
			rope += inst.direction;
			positions.insert(rope.knots[9]);
		}
	}

	positions.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

	const EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 13);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 1);
		assert_eq!(part2(&parse_input(EXAMPLE2)), 36);
	}
}
