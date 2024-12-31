use std::fmt::{Display, Write};

use crate::collections::{Grid, Point, Vector};

#[derive(Clone, PartialEq)]
pub enum Light {
	On,
	Off,
}

impl Display for Light {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Light::On => f.write_char('#'),
			Light::Off => f.write_char('.'),
		}
	}
}

impl From<char> for Light {
	fn from(value: char) -> Self {
		match value {
			'#' => Self::On,
			'.' => Self::Off,
			_ => unreachable!("{value}"),
		}
	}
}

fn neighbors(p: Point, grid: &Grid<Light>) -> u32 {
	let mut neighbors = 0;
	for d in &Vector::DIRECTIONS {
		let pp = p + d;
		if grid.in_bounds(pp) && grid.get(pp) == &Light::On {
			neighbors += 1;
		}
	}
	neighbors
}

fn next_grid(grid: &Grid<Light>) -> Grid<Light> {
	let mut new = grid.clone();

	for y in 0..grid.height {
		for x in 0..grid.width {
			let p = Point(x as i32, y as i32);
			let neighbors = neighbors(p, grid);
			match (grid.get(p), neighbors) {
				(Light::On, 0 | 1 | 4 | 5 | 6 | 7 | 8) => new.set(p, Light::Off),
				(Light::Off, 3) => new.set(p, Light::On),
				_ => (),
			}
		}
	}

	new
}

pub fn parse_input(input: &str) -> Grid<Light> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Light>) -> usize {
	let mut current = input.clone();
	for _ in 0..100 {
		current = next_grid(&current);
	}

	current.count(&Light::On)
}

fn light_corners(grid: &mut Grid<Light>) {
	grid.set(Point(0, 0), Light::On);
	grid.set(Point(0, grid.height as i32 - 1), Light::On);
	grid.set(Point(grid.width as i32 - 1, 0), Light::On);
	grid.set(Point(grid.width as i32 - 1, grid.height as i32 - 1), Light::On);
}

pub fn part2(input: &Grid<Light>) -> usize {
	let mut current = input.clone();
	light_corners(&mut current);

	for _ in 0..100 {
		current = next_grid(&current);
		light_corners(&mut current);
	}

	current.count(&Light::On)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

	#[test]
	fn example_part1() {
		let mut grid = parse_input(EXAMPLE);
		assert_eq!(grid.count(&Light::On), 15);
		grid = next_grid(&grid);
		assert_eq!(grid.count(&Light::On), 11);
		grid = next_grid(&grid);
		assert_eq!(grid.count(&Light::On), 8);
		grid = next_grid(&grid);
		assert_eq!(grid.count(&Light::On), 4);
		grid = next_grid(&grid);
		assert_eq!(grid.count(&Light::On), 4);
	}
}
