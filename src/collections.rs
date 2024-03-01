use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Point {
	pub fn new(x: i32, y: i32) -> Self {
		Point { x, y }
	}
}

impl From<(i32, i32)> for Point {
	fn from(value: (i32, i32)) -> Self {
		Point { x: value.0, y: value.1 }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	North,
	South,
	West,
	East,
}

impl Direction {
	pub fn reverse(&self) -> Direction {
		match self {
			Direction::North => Direction::South,
			Direction::South => Direction::North,
			Direction::West => Direction::East,
			Direction::East => Direction::West,
		}
	}

	pub fn delta(&self) -> (i32, i32) {
		match self {
			Direction::North => (0, -1),
			Direction::South => (0, 1),
			Direction::West => (-1, 0),
			Direction::East => (1, 0),
		}
	}
}

impl Add<Direction> for Point {
	type Output = Point;

	fn add(self, direction: Direction) -> Self::Output {
		let (x, y) = direction.delta();
		Point {
			x: self.x + x,
			y: self.y + y,
		}
	}
}

impl AddAssign<&Direction> for Point {
	fn add_assign(&mut self, rhs: &Direction) {
		let (x, y) = rhs.delta();
		self.x += x;
		self.y += y;
	}
}

#[derive(Debug)]
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

impl<C: Copy> Grid<C> {
	pub fn get(&self, point: Point) -> C {
		self.cells[self.width * point.y as usize + point.x as usize]
	}
}

impl<C: Copy + PartialEq> Grid<C> {
	pub fn find(&self, cell: C) -> Vec<Point> {
		let mut res = Vec::new();
		for y in 0..self.height {
			for x in 0..self.width {
				let point = Point::new(x as i32, y as i32);
				if self.get(point) == cell {
					res.push(point);
				}
			}
		}

		res
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
		assert_eq!(grid.get(Point::new(1, 1)), Cell::On);
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
		assert_eq!(*grid.get(Point::new(1, 1)), 5);
	}
}
