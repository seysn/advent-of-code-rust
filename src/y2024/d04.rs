use std::ops::Range;

use crate::collections::{Grid, Point};

pub fn parse_input(input: &str) -> Grid<char> {
	Grid::new(input)
}

impl Grid<char> {
	fn get_horizontal(&self, x: Range<usize>, y: usize) -> String {
		let mut res = String::new();
		for xx in x {
			let p = Point(xx as i32, y as i32);
			if !self.in_bounds(p) {
				break;
			}
			res.push(self.get(p));
		}
		res
	}

	fn get_vertical(&self, x: usize, y: Range<usize>) -> String {
		let mut res = String::new();
		for yy in y {
			let p = Point(x as i32, yy as i32);
			if !self.in_bounds(p) {
				break;
			}
			res.push(self.get(p));
		}
		res
	}

	fn get_bottom_right(&self, point: Point, len: usize) -> String {
		let mut res = String::new();
		for i in 0..len {
			let p = Point(point.0 + i as i32, point.1 + i as i32);
			if !self.in_bounds(p) {
				break;
			}
			res.push(self.get(p));
		}
		res
	}

	fn get_bottom_left(&self, point: Point, len: usize) -> String {
		let mut res = String::new();
		for i in 0..len {
			let p = Point(point.0 + i as i32, point.1 - i as i32);
			if !self.in_bounds(p) {
				break;
			}
			res.push(self.get(p));
		}
		res
	}

	fn search(&self, word: &str) -> usize {
		let rev = word.chars().rev().collect::<String>();
		let mut res = 0;

		for y in 0..self.height {
			for x in 0..self.width {
				let w = self.get_horizontal(x..x + word.len(), y);
				if w == word || w == rev {
					res += 1;
				}
			}
		}

		for x in 0..self.width {
			for y in 0..self.height {
				let w = self.get_vertical(x, y..y + word.len());
				if w == word || w == rev {
					res += 1;
				}
			}
		}

		for x in 0..self.width {
			for y in 0..self.height {
				let p = Point(x as i32, y as i32);
				let w = self.get_bottom_right(p, word.len());
				if w == word || w == rev {
					res += 1;
				}
				let w = self.get_bottom_left(p, word.len());
				if w == word || w == rev {
					res += 1;
				}
			}
		}

		res
	}

	fn search2(&self) -> usize {
		let mut res = 0;
		for y in 1..(self.height - 1) {
			for x in 1..(self.width - 1) {
				if self.get(Point(x as i32, y as i32)) != 'A' {
					continue;
				}

				let br = self.get(Point(x as i32 + 1, y as i32 + 1));
				let tr = self.get(Point(x as i32 + 1, y as i32 - 1));
				let bl = self.get(Point(x as i32 - 1, y as i32 + 1));
				let tl = self.get(Point(x as i32 - 1, y as i32 - 1));

				let a = (br == 'M' && tl == 'S') || (br == 'S' && tl == 'M');
				let b = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');

				if a && b {
					res += 1;
				}
			}
		}

		res
	}
}

pub fn part1(input: &Grid<char>) -> usize {
	input.search("XMAS")
}

pub fn part2(input: &Grid<char>) -> usize {
	input.search2()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 18);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 9);
	}
}
