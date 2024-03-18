use std::collections::HashMap;

use crate::collections::{Grid, Point};

#[derive(Clone, Copy)]
pub enum Element {
	Number(char),
	Symbol(char),
	None,
}

impl From<char> for Element {
	fn from(value: char) -> Self {
		match value {
			c if c.is_numeric() => Element::Number(c),
			'.' => Element::None,
			c => Element::Symbol(c),
		}
	}
}

impl Grid<Element> {
	fn has_symbol_around(&self, x: usize, y: usize) -> bool {
		for (i, j) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)] {
			let (xx, yy) = (x as i32 + i, y as i32 + j);
			if xx < 0 || yy < 0 || xx >= self.width as i32 || yy >= self.height as i32 {
				continue;
			}

			if let Element::Symbol(_) = self.get(Point(xx, yy)) {
				return true;
			}
		}

		false
	}

	fn is_part_number(&self, line: usize, start: usize, end: usize) -> bool {
		for x in start..=end {
			if self.has_symbol_around(x, line) {
				return true;
			}
		}

		false
	}

	fn has_gear_around(&self, x: usize, y: usize) -> Option<(usize, usize)> {
		for (i, j) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)] {
			let (xx, yy) = (x as i32 + i, y as i32 + j);
			if xx < 0 || yy < 0 || xx >= self.width as i32 || yy >= self.height as i32 {
				continue;
			}

			if let Element::Symbol('*') = self.get(Point(xx, yy)) {
				return Some((xx as usize, yy as usize));
			}
		}

		None
	}

	fn has_gear(&self, line: usize, start: usize, end: usize) -> Option<(usize, usize)> {
		for x in start..=end {
			if let Some(coords) = self.has_gear_around(x, line) {
				return Some(coords);
			}
		}

		None
	}
}

pub fn parse_input(input: &str) -> Grid<Element> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Element>) -> u64 {
	let mut res = 0;
	for line in 0..input.height {
		let mut column = 0;
		let mut tmp = String::new();
		while column < input.width {
			match input.get(Point(column as i32, line as i32)) {
				Element::Number(n) => tmp.push(n),
				Element::Symbol(_) => {
					if !tmp.is_empty() {
						res += tmp.parse::<u64>().unwrap();
						tmp.clear();
					}
				}
				Element::None => {
					if !tmp.is_empty() {
						let start = column - tmp.len();
						let end = column - 1;

						if input.is_part_number(line, start, end) {
							res += tmp.parse::<u64>().unwrap();
						}
						tmp.clear();
					}
				}
			}

			column += 1;
		}

		if !tmp.is_empty() {
			let start = column - tmp.len();
			let end = column - 1;

			if input.is_part_number(line, start, end) {
				res += tmp.parse::<u64>().unwrap();
			}
			tmp.clear();
		}
	}

	res
}

pub fn part2(input: &Grid<Element>) -> u64 {
	let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

	for line in 0..input.height {
		let mut column = 0;
		let mut tmp = String::new();
		while column < input.width {
			match input.get(Point(column as i32, line as i32)) {
				Element::Number(n) => tmp.push(n),
				Element::Symbol(_) | Element::None => {
					if !tmp.is_empty() {
						let start = column - tmp.len();
						let end = column - 1;

						if let Some(coords) = input.has_gear(line, start, end) {
							let value = tmp.parse::<u64>().unwrap();
							gears.entry(coords).and_modify(|v| (*v).push(value)).or_insert(vec![value]);
						}
						tmp.clear();
					}
				}
			}

			column += 1;
		}

		if !tmp.is_empty() {
			let start = column - tmp.len();
			let end = column - 1;

			if let Some(coords) = input.has_gear(line, start, end) {
				let value = tmp.parse::<u64>().unwrap();
				gears.entry(coords).and_modify(|v| (*v).push(value)).or_insert(vec![value]);
			}
			tmp.clear();
		}
	}

	gears
		.values()
		.filter(|gear| gear.len() == 2)
		.map(|gear| gear.iter().product::<u64>())
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

	const EXAMPLE2: &str = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 4361);
		assert_eq!(part1(&parse_input(EXAMPLE2)), 925);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 467835);
		assert_eq!(part2(&parse_input(EXAMPLE2)), 6756);
	}
}
