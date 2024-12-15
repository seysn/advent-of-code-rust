use std::collections::HashSet;

use crate::collections::{Grid, Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Height(u32);

impl From<char> for Height {
	fn from(value: char) -> Self {
		if value == '.' {
			Self(1000)
		} else {
			Self(value as u32 - 48)
		}
	}
}

pub fn parse_input(input: &str) -> Grid<Height> {
	Grid::new(input)
}

fn trail(p: Point, input: &Grid<Height>, tails: &mut Option<HashSet<Point>>) -> u32 {
	let Height(value) = input[p];
	if value == 9 {
		if let Some(t) = tails.as_mut() {
			if t.contains(&p) {
				return 0;
			} else {
				t.insert(p);
				return 1;
			}
		} else {
			return 1;
		}
	}

	let mut res = 0;
	for direction in &Vector::CARDINAL {
		let pp = p + direction;
		if input.in_bounds(pp) && input[pp] == Height(value + 1) {
			res += trail(pp, input, tails);
		}
	}

	res
}

pub fn part1(input: &Grid<Height>) -> u32 {
	input
		.find_all(&Height(0))
		.iter()
		.map(|p| trail(*p, input, &mut Some(HashSet::new())))
		.sum()
}

pub fn part2(input: &Grid<Height>) -> u32 {
	input.find_all(&Height(0)).iter().map(|p| trail(*p, input, &mut None)).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 36);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 81);
	}
}
