use std::collections::HashMap;

use crate::collections::{Grid, Vector};

#[derive(PartialEq)]
pub enum Cell {
	Track,
	Wall,
	Start,
	End,
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
		match value {
			'#' => Self::Wall,
			'S' => Self::Start,
			'E' => Self::End,
			_ => Self::Track,
		}
	}
}

pub fn parse_input(input: &str) -> Grid<Cell> {
	Grid::new(input)
}

// We could use `solve2` for part2 but this one is ~3 times faster for part1
fn solve(input: &Grid<Cell>) -> HashMap<usize, u32> {
	let mut saves = HashMap::new();

	let start = input.find(&Cell::Start).unwrap();
	let end = input.find(&Cell::End).unwrap();
	let mut path = vec![start];

	// Creating normal path
	let mut previous = start;
	let mut current = start;
	while current != end {
		for v in Vector::CARDINAL {
			let next = current + v;
			if next == previous {
				continue;
			}

			let cell = input.get(next);
			if cell == &Cell::Wall {
				continue;
			}

			path.push(next);
			previous = current;
			current = next;
			break;
		}
	}

	// Searching skips
	for (i, current) in path.iter().enumerate() {
		if i > path.len() - 5 {
			// No more solution possible
			break;
		}

		for v in Vector::CARDINAL {
			let jump = v * 2;
			let next = current + jump;
			if !input.in_bounds(next) {
				continue;
			}

			let between = current + v;
			if input.get(between) == &Cell::Track || input.get(next) == &Cell::Wall {
				continue;
			}

			// A cheat cannot be less than `i + 4` from the current position so there's
			// no reason to search before that index
			if let Some(found) = path[i + 4..].iter().position(|point| point == &next) {
				// The save time is `next_index - current_index - 2` but since we start
				// the array at the index `current_index + 4` it becomes `next_index + 2`
				*saves.entry(found + 2).or_default() += 1;
			}
		}
	}

	saves
}

pub fn part1(input: &Grid<Cell>) -> u32 {
	solve(input)
		.iter()
		.filter_map(|(save, n)| if *save >= 100 { Some(n) } else { None })
		.sum()
}

fn solve2(input: &Grid<Cell>) -> HashMap<usize, u32> {
	let mut saves = HashMap::new();

	let start = input.find(&Cell::Start).unwrap();
	let end = input.find(&Cell::End).unwrap();
	let mut path = vec![start];

	// Creating normal path
	let mut previous = start;
	let mut current = start;
	while current != end {
		for v in Vector::CARDINAL {
			let next = current + v;
			if next == previous {
				continue;
			}

			let cell = input.get(next);
			if cell == &Cell::Wall {
				continue;
			}

			path.push(next);
			previous = current;
			current = next;
			break;
		}
	}

	// Searching skips
	for (i, current) in path.iter().enumerate() {
		for (j, next) in path.iter().enumerate().skip(i + 50) {
			let distance = current.vector(next).manhattan_distance() as usize;
			if distance <= 20 {
				*saves.entry(j - i - distance).or_default() += 1;
			}
		}
	}

	saves
}

pub fn part2(input: &Grid<Cell>) -> u32 {
	solve2(input)
		.iter()
		.filter_map(|(save, n)| if *save >= 100 { Some(n) } else { None })
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

	#[test]
	fn example_part1() {
		let saves = solve(&parse_input(EXAMPLE));
		assert_eq!(saves.get(&2), Some(&14));
		assert_eq!(saves.get(&4), Some(&14));
		assert_eq!(saves.get(&6), Some(&2));
		assert_eq!(saves.get(&8), Some(&4));
		assert_eq!(saves.get(&10), Some(&2));
		assert_eq!(saves.get(&12), Some(&3));
		assert_eq!(saves.get(&20), Some(&1));
		assert_eq!(saves.get(&36), Some(&1));
		assert_eq!(saves.get(&38), Some(&1));
		assert_eq!(saves.get(&40), Some(&1));
		assert_eq!(saves.get(&64), Some(&1));
	}

	#[test]
	fn example_part2() {
		let saves = solve2(&parse_input(EXAMPLE));
		assert_eq!(saves.get(&50), Some(&32));
		assert_eq!(saves.get(&52), Some(&31));
		assert_eq!(saves.get(&54), Some(&29));
	}
}
