use std::fmt::{Display, Write};

use crate::collections::{Grid, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
	Empty,
	Roll,
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
		match value {
			'@' => Self::Roll,
			'.' => Self::Empty,
			_ => panic!("Cell {value} is not valid"),
		}
	}
}

impl Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(match self {
			Cell::Empty => '.',
			Cell::Roll => '@',
		})
	}
}

pub fn parse_input(input: &str) -> Grid<Cell> {
	Grid::new(input)
}

fn is_roll_accessible(x: usize, y: usize, input: &Grid<Cell>) -> bool {
	if input.get(Point(x as i32, y as i32)) == &Cell::Empty {
		return false;
	}

	let mut rolls = 0;

	for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
		let p = Point(x as i32 + dx, y as i32 + dy);
		if !input.in_bounds(p) {
			continue;
		}

		if input.get(p) == &Cell::Roll {
			rolls += 1;
		}

		if rolls >= 4 {
			return false;
		}
	}

	rolls < 4
}

pub fn part1(input: &Grid<Cell>) -> usize {
	let mut res = 0;
	for y in 0..input.height {
		for x in 0..input.width {
			if is_roll_accessible(x, y, input) {
				res += 1;
			}
		}
	}

	res
}

pub fn part2(input: &Grid<Cell>) -> usize {
	let mut grid = input.clone();
	let mut res = 0;

	loop {
		let mut to_remove = Vec::new();
		for y in 0..grid.height {
			for x in 0..grid.width {
				if is_roll_accessible(x, y, &grid) {
					to_remove.push(Point(x as i32, y as i32));
				}
			}
		}

		if to_remove.is_empty() {
			break;
		}

		res += to_remove.len();
		for p in to_remove {
			grid.set(p, Cell::Empty);
		}
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 13);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 43);
	}
}
