use crate::collections::{Grid, Point, Vector};

pub fn parse_input(input: &str) -> Grid<char> {
	Grid::new(input)
}

pub fn part1(input: &Grid<char>) -> usize {
	let mut res = 0;

	for y in 0..input.height {
		for x in 0..input.width {
			let p = Point(x as i32, y as i32);
			if input[p] != 'X' {
				continue;
			}

			for v in &Vector::DIRECTIONS {
				if input.in_bounds(p + v * 3) && input[p + v] == 'M' && input[p + v * 2] == 'A' && input[p + v * 3] == 'S' {
					res += 1;
				}
			}
		}
	}

	res
}

pub fn part2(input: &Grid<char>) -> usize {
	let mut res = 0;
	for y in 1..(input.height - 1) {
		for x in 1..(input.width - 1) {
			let p = Point(x as i32, y as i32);
			if input[p] != 'A' {
				continue;
			}

			let se = input[p + Vector::SOUTH_EAST];
			let ne = input[p + Vector::NORTH_EAST];
			let sw = input[p + Vector::SOUTH_WEST];
			let nw = input[p + Vector::NORTH_WEST];

			let a = (se == 'M' && nw == 'S') || (se == 'S' && nw == 'M');
			let b = (ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M');

			if a && b {
				res += 1;
			}
		}
	}

	res
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
