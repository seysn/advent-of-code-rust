use std::{
	collections::{HashMap, HashSet},
	iter,
};

use crate::collections::{Grid, Point, Vector};

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
	direction: Vector,
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
	fn out_of_bounds(&self, point: &Point, direction: &Vector) -> bool {
		!self.in_bounds(point + direction)
	}

	fn split(
		&self,
		point: Point,
		direction: &Vector,
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
		let mut direction = start.direction;
		loop {
			res.insert(point);
			match (self.get(point), &direction) {
				(Tile::MirrorFront, &Vector::NORTH) => direction = Vector::EAST,
				(Tile::MirrorFront, &Vector::SOUTH) => direction = Vector::WEST,
				(Tile::MirrorFront, &Vector::WEST) => direction = Vector::SOUTH,
				(Tile::MirrorFront, &Vector::EAST) => direction = Vector::NORTH,
				(Tile::MirrorBack, &Vector::NORTH) => direction = Vector::WEST,
				(Tile::MirrorBack, &Vector::SOUTH) => direction = Vector::EAST,
				(Tile::MirrorBack, &Vector::WEST) => direction = Vector::NORTH,
				(Tile::MirrorBack, &Vector::EAST) => direction = Vector::SOUTH,
				(Tile::SplitterH, &Vector::NORTH | &Vector::SOUTH) => {
					if splitter_used.contains(&point) {
						break;
					}
					splitter_used.push(point);

					if !self.out_of_bounds(&point, &Vector::WEST) {
						res.extend(self.split(point, &Vector::WEST, cache, splitter_used));
					}
					if !self.out_of_bounds(&point, &Vector::EAST) {
						res.extend(self.split(point, &Vector::EAST, cache, splitter_used));
					}
					break;
				}
				(Tile::SplitterV, &Vector::WEST | &Vector::EAST) => {
					if splitter_used.contains(&point) {
						break;
					}
					splitter_used.push(point);

					if !self.out_of_bounds(&point, &Vector::NORTH) {
						res.extend(self.split(point, &Vector::NORTH, cache, splitter_used));
					}
					if !self.out_of_bounds(&point, &Vector::SOUTH) {
						res.extend(self.split(point, &Vector::SOUTH, cache, splitter_used));
					}
					break;
				}
				(_, _) => {}
			}

			if self.out_of_bounds(&point, &direction) {
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
				direction: Vector::EAST,
			},
			&mut cache,
			&mut vec![],
		)
		.len()
}

fn get_max<I, J>(input: &Grid<Tile>, xs: I, ys: J, direction: Vector, cache: &mut HashMap<Start, HashSet<Point>>) -> usize
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

	res = res.max(get_max(input, 0..input.width, iter::repeat(0), Vector::SOUTH, &mut cache));
	res = res.max(get_max(
		input,
		0..input.width,
		iter::repeat(input.height - 1),
		Vector::NORTH,
		&mut cache,
	));
	res = res.max(get_max(input, iter::repeat(0), 0..input.height, Vector::NORTH, &mut cache));
	res = res.max(get_max(
		input,
		iter::repeat(input.width - 1),
		0..input.height,
		Vector::WEST,
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
