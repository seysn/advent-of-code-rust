#[derive(Debug)]
pub enum Direction {
	Left(i32),
	Right(i32),
}

impl Direction {
	fn blocks(&self) -> i32 {
		*match self {
			Direction::Left(blocks) => blocks,
			Direction::Right(blocks) => blocks,
		}
	}
}

enum Face {
	North,
	South,
	East,
	West,
}

impl Face {
	fn rotate(&self, direction: &Direction) -> Face {
		match self {
			Face::North => match direction {
				Direction::Left(_) => Face::West,
				Direction::Right(_) => Face::East,
			},
			Face::South => match direction {
				Direction::Left(_) => Face::East,
				Direction::Right(_) => Face::West,
			},
			Face::East => match direction {
				Direction::Left(_) => Face::North,
				Direction::Right(_) => Face::South,
			},
			Face::West => match direction {
				Direction::Left(_) => Face::South,
				Direction::Right(_) => Face::North,
			},
		}
	}
}

struct Position {
	face: Face,
	x: i32,
	y: i32,
}

impl Position {
	fn new() -> Position {
		Position {
			face: Face::North,
			x: 0,
			y: 0,
		}
	}

	fn walk(&mut self, direction: &Direction) {
		let blocks = direction.blocks();
		self.face = self.face.rotate(direction);
		match self.face {
			Face::North => self.x += blocks,
			Face::South => self.x -= blocks,
			Face::East => self.y += blocks,
			Face::West => self.y -= blocks,
		}
	}
}

pub fn parse_input(input: &str) -> Vec<Direction> {
	input
		.split(", ")
		.map(|l| {
			let mut it = l.chars();
			let direction = it.next().unwrap();
			let blocks = it.collect::<String>().parse::<i32>().unwrap();

			match direction {
				'L' => Direction::Left(blocks),
				'R' => Direction::Right(blocks),
				_ => unreachable!(),
			}
		})
		.collect()
}

pub fn part1(input: &[Direction]) -> i32 {
	let mut position = Position::new();
	for direction in input {
		position.walk(direction);
	}
	position.x.abs() + position.y.abs()
}

pub fn part2(input: &[Direction]) -> i32 {
	let mut visited: Vec<(i32, i32)> = Vec::new();
	let mut position = Position::new();
	for direction in input {
		let (x, y) = (position.x, position.y);
		position.walk(direction);

		let range: Vec<(i32, i32)> = if x < position.x {
			(x + 1..position.x + 1).map(|x| (x, y)).collect()
		} else if x > position.x {
			(position.x..x).map(|x| (x, y)).collect()
		} else if y < position.y {
			(y + 1..position.y + 1).map(|y| (x, y)).collect()
		} else if y > position.y {
			(position.y..y).map(|y| (x, y)).collect()
		} else {
			unreachable!()
		};

		if let Some(pos) = range.iter().find_map(|pos| visited.iter().find(|&elem| elem == pos)) {
			return pos.0.abs() + pos.1.abs();
		}

		visited.extend(range);
	}

	unreachable!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input("R2, L3")), 5);
		assert_eq!(part1(&parse_input("R2, R2, R2")), 2);
		assert_eq!(part1(&parse_input("R5, L5, R5, R3")), 12);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input("R8, R4, R4, R8")), 4);
	}
}
