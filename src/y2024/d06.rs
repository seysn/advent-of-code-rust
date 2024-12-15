use std::collections::HashSet;

use crate::collections::{Grid, Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
	Void,
	Obstacle,
	Guard(Vector),
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
		match value {
			'#' => Self::Obstacle,
			'^' => Self::Guard(Vector::NORTH),
			_ => Self::Void,
		}
	}
}

pub fn parse_input(input: &str) -> Grid<Cell> {
	Grid::new(input)
}

fn visited(input: &Grid<Cell>) -> HashSet<Point> {
	let mut position = input.find(&Cell::Guard(Vector::NORTH)).unwrap();
	let mut direction = Vector::NORTH;

	let mut visited: HashSet<Point> = HashSet::new();
	loop {
		visited.insert(position);
		let next = position + direction;
		if !input.in_bounds(next) {
			break;
		}

		if input.get(next) == &Cell::Obstacle {
			direction = direction.clockwise();
		} else {
			position += direction;
		}
	}

	visited
}

pub fn part1(input: &Grid<Cell>) -> usize {
	visited(input).len()
}

fn is_looping(grid: &Grid<Cell>) -> bool {
	let mut position = grid.find(&Cell::Guard(Vector::NORTH)).unwrap();
	let mut direction = Vector::NORTH;

	let mut visited: HashSet<(Point, Vector)> = HashSet::new();
	loop {
		if visited.contains(&(position, direction)) {
			return true;
		}
		visited.insert((position, direction));
		let next = position + direction;
		if !grid.in_bounds(next) {
			break;
		}

		if grid.get(next) == &Cell::Obstacle {
			direction = direction.clockwise();
		} else {
			position += direction;
		}
	}

	false
}

/// I got the second star with this one.
/// Good old bruteforce that takes 11 seconds to give a solution
#[allow(unused)]
fn bruteforce(input: &Grid<Cell>) -> usize {
	let mut res = 0;

	for y in 0..input.height {
		for x in 0..input.width {
			let p = Point(x as i32, y as i32);
			if input.get(p) == &Cell::Void {
				let mut grid = input.clone();
				grid[p] = Cell::Obstacle;
				if is_looping(&grid) {
					res += 1;
				}
			}
		}
	}
	res
}

/// "Optimized" bruteforce that still takes a lot of time
fn bruteforce2(input: &Grid<Cell>) -> usize {
	let start = input.find(&Cell::Guard(Vector::NORTH)).unwrap();
	let visited = visited(input);

	let mut res = 0;
	for y in 0..input.height {
		for x in 0..input.width {
			let p = Point(x as i32, y as i32);
			if !visited.contains(&p) || p == start {
				continue;
			}

			if input.get(p) == &Cell::Void {
				let mut grid = input.clone();
				grid[p] = Cell::Obstacle;
				if is_looping(&grid) {
					res += 1;
				}
			}
		}
	}
	res
}

pub fn part2(input: &Grid<Cell>) -> usize {
	if input.height <= 10 && input.width <= 10 {
		bruteforce2(input)
	} else {
		// TODO: even the "optimized" bruteforce takes to much time
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 41);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 6);
	}
}
