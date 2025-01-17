use std::fmt::{Display, Write};

use crate::collections::{Grid, Point};

#[derive(Debug)]
pub enum Operation {
	Rect(i32, i32),
	RotateRow(i32, usize),
	RotateColumn(i32, usize),
}

pub fn parse_input(input: &str) -> Vec<Operation> {
	input
		.lines()
		.map(|l| {
			if let Some(s) = l.strip_prefix("rect ") {
				let (x, y) = s.split_once('x').unwrap();
				Operation::Rect(x.parse().unwrap(), y.parse().unwrap())
			} else if let Some(s) = l.strip_prefix("rotate row y=") {
				let (x, n) = s.split_once(" by ").unwrap();
				Operation::RotateRow(x.parse().unwrap(), n.parse().unwrap())
			} else if let Some(s) = l.strip_prefix("rotate column x=") {
				let (x, n) = s.split_once(" by ").unwrap();
				Operation::RotateColumn(x.parse().unwrap(), n.parse().unwrap())
			} else {
				unreachable!("{l}");
			}
		})
		.collect()
}

#[derive(Clone, PartialEq)]
enum Pixel {
	On,
	Off,
}

struct Screen {
	grid: Grid<Pixel>,
}

impl Display for Pixel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Pixel::On => f.write_char('#'),
			Pixel::Off => f.write_char('.'),
		}
	}
}

impl Display for Screen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.grid, f)
	}
}

impl Screen {
	fn new(width: usize, height: usize) -> Self {
		Self {
			grid: Grid::fill(&Pixel::Off, width, height),
		}
	}

	fn exec(&mut self, operation: &Operation) {
		match operation {
			Operation::Rect(x, y) => {
				for yy in 0..*y {
					for xx in 0..*x {
						self.grid.set(Point(xx, yy), Pixel::On);
					}
				}
			}
			Operation::RotateRow(y, n) => {
				let mut v = Vec::with_capacity(self.grid.width);
				for x in 0..self.grid.width {
					v.push(self.grid.get(Point(x as i32, *y)).clone());
				}
				v.rotate_right(*n);
				for (x, px) in v.iter().enumerate() {
					self.grid.set(Point(x as i32, *y), px.clone());
				}
			}
			Operation::RotateColumn(x, n) => {
				let mut v = Vec::with_capacity(self.grid.height);
				for y in 0..self.grid.height {
					v.push(self.grid.get(Point(*x, y as i32)).clone());
				}
				v.rotate_right(*n);
				for (y, px) in v.iter().enumerate() {
					self.grid.set(Point(*x, y as i32), px.clone());
				}
			}
		}
	}

	fn count_lit(&self) -> usize {
		self.grid.count(&Pixel::On)
	}
}

pub fn part1(input: &[Operation]) -> usize {
	let mut screen = Screen::new(50, 6);
	for op in input {
		screen.exec(op);
	}

	screen.count_lit()
}

pub fn part2(input: &[Operation]) -> String {
	let mut screen = Screen::new(50, 6);
	for op in input {
		screen.exec(op);
	}

	format!("\n{screen}")
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

	#[test]
	fn example_part1() {
		let mut screen = Screen::new(7, 3);
		assert_eq!(&format!("{screen}"), ".......\n.......\n.......\n");
		screen.exec(&Operation::Rect(3, 2));
		assert_eq!(&format!("{screen}"), "###....\n###....\n.......\n");
		screen.exec(&Operation::RotateColumn(1, 1));
		assert_eq!(&format!("{screen}"), "#.#....\n###....\n.#.....\n");
		screen.exec(&Operation::RotateRow(0, 4));
		assert_eq!(&format!("{screen}"), "....#.#\n###....\n.#.....\n");
		screen.exec(&Operation::RotateColumn(1, 1));
		assert_eq!(&format!("{screen}"), ".#..#.#\n#.#....\n.#.....\n");

		assert_eq!(part1(&parse_input(EXAMPLE)), 6);
	}
}
