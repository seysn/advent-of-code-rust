use std::ops::{Deref, DerefMut};

use crate::collections::{Grid, Point};

#[derive(Clone, Copy)]
pub struct Number(u32);

impl From<char> for Number {
	fn from(value: char) -> Self {
		Number(value.to_digit(10).unwrap())
	}
}

impl Deref for Number {
	type Target = u32;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Number {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Grid<Number> {
	fn inc(&mut self, x: usize, y: usize) {
		*self.cells[self.width * y + x] += 1;
	}

	fn scenic_score(&self, x: usize, y: usize) -> u32 {
		let value = *self.get(Point(x as i32, y as i32));
		let mut res = 1;
		let mut cpt = 0;

		// Up
		for j in (0..y).rev() {
			cpt += 1;
			if **self.get(Point(x as i32, j as i32)) >= *value {
				break;
			}
		}
		res *= cpt;
		cpt = 0;

		// Down
		for j in y + 1..self.height {
			cpt += 1;
			if **self.get(Point(x as i32, j as i32)) >= *value {
				break;
			}
		}
		res *= cpt;
		cpt = 0;

		// Left
		for i in (0..x).rev() {
			cpt += 1;
			if **self.get(Point(i as i32, y as i32)) >= *value {
				break;
			}
		}
		res *= cpt;
		cpt = 0;

		// Right
		for i in x + 1..self.width {
			cpt += 1;
			if **self.get(Point(i as i32, y as i32)) >= *value {
				break;
			}
		}
		res *= cpt;

		res
	}
}

pub fn parse_input(input: &str) -> Grid<Number> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Number>) -> usize {
	let mut visibles = Grid {
		cells: vec![Number(0); input.width * input.height],
		width: input.width,
		height: input.height,
	};

	// Inc top and bottom borders
	for x in 0..input.width {
		visibles.inc(x, 0);
		visibles.inc(x, input.height - 1);
	}

	// Inc left and right borders
	for y in 0..input.width {
		visibles.inc(0, y);
		visibles.inc(input.width - 1, y);
	}

	let mut max;
	for x in 1..input.width - 1 {
		// From top to bottom
		max = *input.get(Point(x as i32, 0));
		for y in 1..input.height {
			let tree = *input.get(Point(x as i32, y as i32));
			if *tree > *max {
				visibles.inc(x, y);
				max = tree;
			}
		}

		// From bottom to top
		max = *input.get(Point(x as i32, input.height as i32 - 1));
		for y in (0..input.height - 1).rev() {
			let tree = *input.get(Point(x as i32, y as i32));
			if *tree > *max {
				visibles.inc(x, y);
				max = tree;
			}
		}
	}

	for y in 1..input.height - 1 {
		// From left to right
		max = *input.get(Point(0, y as i32));
		for x in 1..input.width {
			let tree = *input.get(Point(x as i32, y as i32));
			if *tree > *max {
				visibles.inc(x, y);
				max = tree;
			}
		}

		// From right to left
		max = *input.get(Point(input.width as i32 - 1, y as i32));
		for x in (0..input.width - 1).rev() {
			let tree = *input.get(Point(x as i32, y as i32));
			if *tree > *max {
				visibles.inc(x, y);
				max = tree;
			}
		}
	}

	visibles.cells.iter().filter(|&&x| *x > 0).count()
}

pub fn part2(input: &Grid<Number>) -> u32 {
	let mut visibles = Grid {
		cells: vec![Number(1); input.width * input.height],
		width: input.width,
		height: input.height,
	};

	for x in 1..input.width - 1 {
		for y in 1..input.height - 1 {
			visibles.set(Point(x as i32, y as i32), Number(input.scenic_score(x, y)));
		}
	}

	visibles.cells.iter().map(|&n| *n).max().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "30373
25512
65332
33549
35390";

	#[test]
	fn example_get() {
		let grid = parse_input(EXAMPLE);
		assert_eq!(**grid.get(Point(0, 0)), 3);
		assert_eq!(**grid.get(Point(1, 1)), 5);
		assert_eq!(**grid.get(Point(3, 0)), 7);
		assert_eq!(**grid.get(Point(0, 2)), 6);
	}

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 21);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 8);
	}
}
