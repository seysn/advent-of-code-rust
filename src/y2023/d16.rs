use std::{
	collections::{HashMap, HashSet},
	iter,
	ops::{Add, AddAssign},
};

#[derive(Clone, Copy)]
enum Tile {
	Space,
	MirrorFront,
	MirrorBack,
	SplitterH,
	SplitterV,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	North,
	South,
	West,
	East,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
	x: usize,
	y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Start {
	position: Position,
	direction: Direction,
}

pub struct Grid {
	tiles: Vec<Tile>,
	width: usize,
	height: usize,
}

impl From<char> for Tile {
	fn from(value: char) -> Self {
		match value {
			'/' => Self::MirrorFront,
			'\\' => Self::MirrorBack,
			'-' => Self::SplitterH,
			'|' => Self::SplitterV,
			_ => Self::Space,
		}
	}
}

impl Direction {
	fn delta(&self) -> (i32, i32) {
		match self {
			Direction::North => (0, -1),
			Direction::South => (0, 1),
			Direction::West => (-1, 0),
			Direction::East => (1, 0),
		}
	}
}

impl Add<&Direction> for Position {
	type Output = Self;

	fn add(self, rhs: &Direction) -> Self::Output {
		let (x, y) = rhs.delta();
		Position {
			x: (self.x as i32 + x) as usize,
			y: (self.y as i32 + y) as usize,
		}
	}
}

impl AddAssign<&Direction> for Position {
	fn add_assign(&mut self, rhs: &Direction) {
		let (x, y) = rhs.delta();
		self.x = (self.x as i32 + x) as usize;
		self.y = (self.y as i32 + y) as usize;
	}
}

impl Grid {
	fn get(&self, position: &Position) -> Tile {
		self.tiles[self.width * position.y + position.x]
	}

	fn out_of_bounds(&self, position: &Position, direction: &Direction) -> bool {
		match direction {
			Direction::North => position.y == 0,
			Direction::South => position.y == self.height - 1,
			Direction::West => position.x == 0,
			Direction::East => position.x == self.width - 1,
		}
	}

	fn split(
		&self,
		position: Position,
		direction: &Direction,
		cache: &mut HashMap<Start, HashSet<Position>>,
		splitter_used: &mut Vec<Position>,
	) -> HashSet<Position> {
		let new_pos = position + direction;
		let new_start = Start {
			position: new_pos,
			direction: *direction,
		};

		if let Some(set) = cache.get(&new_start) {
			set.clone()
		} else {
			let set = self.energize(new_start, cache, splitter_used);
			cache.insert(new_start, set.clone());
			set
		}
	}

	fn energize(
		&self,
		start: Start,
		cache: &mut HashMap<Start, HashSet<Position>>,
		splitter_used: &mut Vec<Position>,
	) -> HashSet<Position> {
		let mut res = HashSet::new();
		let mut position = start.position;
		let mut direction = &start.direction;
		loop {
			res.insert(position);
			match (self.get(&position), direction) {
				(Tile::MirrorFront, Direction::North) => direction = &Direction::East,
				(Tile::MirrorFront, Direction::South) => direction = &Direction::West,
				(Tile::MirrorFront, Direction::West) => direction = &Direction::South,
				(Tile::MirrorFront, Direction::East) => direction = &Direction::North,
				(Tile::MirrorBack, Direction::North) => direction = &Direction::West,
				(Tile::MirrorBack, Direction::South) => direction = &Direction::East,
				(Tile::MirrorBack, Direction::West) => direction = &Direction::North,
				(Tile::MirrorBack, Direction::East) => direction = &Direction::South,
				(Tile::SplitterH, Direction::North | Direction::South) => {
					if splitter_used.contains(&position) {
						break;
					}
					splitter_used.push(position);

					if !self.out_of_bounds(&position, &Direction::West) {
						res.extend(self.split(position, &Direction::West, cache, splitter_used));
					}
					if !self.out_of_bounds(&position, &Direction::East) {
						res.extend(self.split(position, &Direction::East, cache, splitter_used));
					}
					break;
				}
				(Tile::SplitterV, Direction::West | Direction::East) => {
					if splitter_used.contains(&position) {
						break;
					}
					splitter_used.push(position);

					if !self.out_of_bounds(&position, &Direction::North) {
						res.extend(self.split(position, &Direction::North, cache, splitter_used));
					}
					if !self.out_of_bounds(&position, &Direction::South) {
						res.extend(self.split(position, &Direction::South, cache, splitter_used));
					}
					break;
				}
				(_, _) => {}
			}

			if self.out_of_bounds(&position, direction) {
				break;
			}
			position += direction;
		}

		res
	}
}

pub fn parse_input(input: &str) -> Grid {
	let tiles: Vec<Tile> = input.chars().filter(|c| !c.is_whitespace()).map(Tile::from).collect();
	let width = input.lines().next().unwrap().len();
	let height = tiles.len() / width;

	Grid { tiles, width, height }
}

pub fn part1(input: &Grid) -> usize {
	let mut cache = HashMap::new();
	input
		.energize(
			Start {
				position: Position { x: 0, y: 0 },
				direction: Direction::East,
			},
			&mut cache,
			&mut vec![],
		)
		.len()
}

fn get_max<I, J>(input: &Grid, xs: I, ys: J, direction: Direction, cache: &mut HashMap<Start, HashSet<Position>>) -> usize
where
	I: Iterator<Item = usize>,
	J: Iterator<Item = usize>,
{
	let mut res = 0;
	for (x, y) in xs.zip(ys) {
		res = res.max(
			input
				.energize(
					Start {
						position: Position { x, y },
						direction,
					},
					cache,
					&mut vec![],
				)
				.len(),
		);
	}
	res
}

pub fn part2(input: &Grid) -> usize {
	let mut cache = HashMap::new();
	let mut res = 0;

	res = res.max(get_max(input, 0..input.width, iter::repeat(0), Direction::South, &mut cache));
	res = res.max(get_max(
		input,
		0..input.width,
		iter::repeat(input.height - 1),
		Direction::North,
		&mut cache,
	));
	res = res.max(get_max(input, iter::repeat(0), 0..input.height, Direction::North, &mut cache));
	res = res.max(get_max(
		input,
		iter::repeat(input.width - 1),
		0..input.height,
		Direction::West,
		&mut cache,
	));

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 46);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 51);
	}
}
