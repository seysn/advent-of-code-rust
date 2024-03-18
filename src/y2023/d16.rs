use std::{
	collections::{HashMap, HashSet},
	iter,
};

use crate::collections::{Direction, Grid, Point};

#[derive(Clone, Copy)]
pub enum Tile {
	Space,
	MirrorFront,
	MirrorBack,
	SplitterH,
	SplitterV,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Start {
	point: Point,
	direction: Direction,
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

impl Grid<Tile> {
	fn out_of_bounds(&self, point: &Point, direction: &Direction) -> bool {
		match direction {
			Direction::North => point.1 == 0,
			Direction::South => point.1 == self.height as i32 - 1,
			Direction::West => point.0 == 0,
			Direction::East => point.0 == self.width as i32 - 1,
		}
	}

	fn split(
		&self,
		point: Point,
		direction: &Direction,
		cache: &mut HashMap<Start, HashSet<Point>>,
		splitter_used: &mut Vec<Point>,
	) -> HashSet<Point> {
		let new_pos = point + *direction;
		let new_start = Start {
			point: new_pos,
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

	fn energize(&self, start: Start, cache: &mut HashMap<Start, HashSet<Point>>, splitter_used: &mut Vec<Point>) -> HashSet<Point> {
		let mut res = HashSet::new();
		let mut point = start.point;
		let mut direction = &start.direction;
		loop {
			res.insert(point);
			match (self.get(point), direction) {
				(Tile::MirrorFront, Direction::North) => direction = &Direction::East,
				(Tile::MirrorFront, Direction::South) => direction = &Direction::West,
				(Tile::MirrorFront, Direction::West) => direction = &Direction::South,
				(Tile::MirrorFront, Direction::East) => direction = &Direction::North,
				(Tile::MirrorBack, Direction::North) => direction = &Direction::West,
				(Tile::MirrorBack, Direction::South) => direction = &Direction::East,
				(Tile::MirrorBack, Direction::West) => direction = &Direction::North,
				(Tile::MirrorBack, Direction::East) => direction = &Direction::South,
				(Tile::SplitterH, Direction::North | Direction::South) => {
					if splitter_used.contains(&point) {
						break;
					}
					splitter_used.push(point);

					if !self.out_of_bounds(&point, &Direction::West) {
						res.extend(self.split(point, &Direction::West, cache, splitter_used));
					}
					if !self.out_of_bounds(&point, &Direction::East) {
						res.extend(self.split(point, &Direction::East, cache, splitter_used));
					}
					break;
				}
				(Tile::SplitterV, Direction::West | Direction::East) => {
					if splitter_used.contains(&point) {
						break;
					}
					splitter_used.push(point);

					if !self.out_of_bounds(&point, &Direction::North) {
						res.extend(self.split(point, &Direction::North, cache, splitter_used));
					}
					if !self.out_of_bounds(&point, &Direction::South) {
						res.extend(self.split(point, &Direction::South, cache, splitter_used));
					}
					break;
				}
				(_, _) => {}
			}

			if self.out_of_bounds(&point, direction) {
				break;
			}
			point += direction;
		}

		res
	}
}

pub fn parse_input(input: &str) -> Grid<Tile> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Tile>) -> usize {
	let mut cache = HashMap::new();
	input
		.energize(
			Start {
				point: Point(0, 0),
				direction: Direction::East,
			},
			&mut cache,
			&mut vec![],
		)
		.len()
}

fn get_max<I, J>(input: &Grid<Tile>, xs: I, ys: J, direction: Direction, cache: &mut HashMap<Start, HashSet<Point>>) -> usize
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
						point: Point(x as i32, y as i32),
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

pub fn part2(input: &Grid<Tile>) -> usize {
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
