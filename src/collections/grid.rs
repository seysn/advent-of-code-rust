use std::{
	fmt::{Display, Write},
	ops::{Index, IndexMut},
};

use super::Point;

#[derive(Debug, Clone)]
pub struct Grid<C> {
	pub cells: Vec<C>,
	pub width: usize,
	pub height: usize,
}

impl<C: From<char>> Grid<C> {
	pub fn new(s: &str) -> Self {
		let cells: Vec<C> = s.chars().filter(|c| !c.is_whitespace()).map(|c| C::from(c)).collect();
		let width = s.lines().next().unwrap().len();
		let height = cells.len() / width;

		Self { cells, width, height }
	}
}

impl<C> Grid<C> {
	pub fn swap(&mut self, a: Point, b: Point) {
		let a = self.width * a.1 as usize + a.0 as usize;
		let b = self.width * b.1 as usize + b.0 as usize;
		self.cells.swap(a, b);
	}
}

impl<C: Copy> Grid<C> {
	#[allow(unused)]
	pub fn fill(cell: C, width: usize, height: usize) -> Self {
		Self {
			cells: vec![cell; width * height],
			width,
			height,
		}
	}

	pub fn get(&self, point: Point) -> C {
		self[point]
	}

	pub fn in_bounds(&self, point: Point) -> bool {
		point.0 >= 0 && point.1 >= 0 && point.0 < self.width as i32 && point.1 < self.height as i32
	}
}

impl<C: Copy + PartialEq> Grid<C> {
	pub fn find(&self, cell: C) -> Vec<Point> {
		let mut res = Vec::new();
		for y in 0..self.height {
			for x in 0..self.width {
				let point = Point(x as i32, y as i32);
				if self.get(point) == cell {
					res.push(point);
				}
			}
		}

		res
	}
}

impl<C> Index<Point> for Grid<C> {
	type Output = C;

	fn index(&self, point: Point) -> &Self::Output {
		&self.cells[self.width * point.1 as usize + point.0 as usize]
	}
}

impl<C> IndexMut<Point> for Grid<C> {
	fn index_mut(&mut self, point: Point) -> &mut Self::Output {
		&mut self.cells[self.width * point.1 as usize + point.0 as usize]
	}
}

impl<C: Display> Display for Grid<C> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 0..self.height {
			for x in 0..self.width {
				self.cells[self.width * y + x].fmt(f)?;
			}
			f.write_char('\n')?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::ops::Deref;

	use super::*;

	#[test]
	fn test_enum() {
		#[derive(Debug, Clone, Copy, PartialEq)]
		enum Cell {
			On,
			Off,
		}

		impl From<char> for Cell {
			fn from(value: char) -> Self {
				match value {
					'Y' => Cell::On,
					'N' => Cell::Off,
					_ => unreachable!(),
				}
			}
		}

		let input = "YNY\nNYN";
		let grid: Grid<Cell> = Grid::new(input);

		assert_eq!(grid.height, 2);
		assert_eq!(grid.width, 3);
		assert_eq!(grid.get(Point(1, 1)), Cell::On);
	}

	#[test]
	fn test_number() {
		#[derive(Debug, Clone, Copy, PartialEq)]
		struct Number(u32);

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

		let input = "123\n456";
		let grid: Grid<Number> = Grid::new(input);

		assert_eq!(grid.height, 2);
		assert_eq!(grid.width, 3);
		assert_eq!(*grid.get(Point(1, 1)), 5);
	}
}
