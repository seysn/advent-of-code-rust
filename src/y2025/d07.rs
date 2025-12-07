use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use crate::collections::{Grid, Point, Vector};

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
	Empty,
	Start,
	Splitter,
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
		match value {
			'.' => Self::Empty,
			'S' => Self::Start,
			'^' => Self::Splitter,
			_ => panic!("Cell '{value}' is not valid"),
		}
	}
}

pub fn parse_input(input: &str) -> Grid<Cell> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Cell>) -> usize {
	let start = input.find(&Cell::Start).unwrap();

	let mut tails = VecDeque::new();
	tails.push_back(start);

	// The goal is just to count to number of used splitters, so we can keep a set of
	// used splitters positions.
	let mut splitted = HashSet::new();
	while let Some(tail) = tails.pop_front() {
		let down = tail + Vector::SOUTH;
		if !input.in_bounds(down) {
			continue;
		}

		if input.get(down) == &Cell::Splitter {
			if splitted.contains(&down) {
				// On the real input, there are too many differents paths, so we need to skip
				// if we already went there to save time
				continue;
			}

			splitted.insert(down);

			let left = down + Vector::WEST;
			if input.in_bounds(left) {
				tails.push_back(left);
			}

			let right = down + Vector::EAST;
			if input.in_bounds(right) {
				tails.push_back(right);
			}
		} else {
			tails.push_back(down);
		}
	}

	splitted.len()
}

struct Tail {
	point: Point,
	last_splitter: Option<Point>,
}

pub fn part2(input: &Grid<Cell>) -> usize {
	let start = input.find(&Cell::Start).unwrap();

	let mut tails = VecDeque::new();
	tails.push_back(Tail {
		point: start,
		last_splitter: None,
	});

	// The goal is to emulate some path and keeping a cache of visited splitter just like in part 1
	// but also include the number of time we visited a splitter to "emulate" every path, this way
	// we can increment each visited value by the last splitter visited value.
	// And it works because the tails are ordered by height naturally.
	// For exemple, if we visit a splitter B and we got there by a splitter A visited a total of 4
	// times, it means that there are a total of 4 timelines up to splitter A, so we have to add
	// visited value of splitter B by 4 because 4 timelines would have come here.
	let mut timelines = 0;
	let mut splitters = HashMap::new();
	while let Some(tail) = tails.pop_front() {
		let down = tail.point + Vector::SOUTH;
		if !input.in_bounds(down) {
			timelines += splitters.get(&tail.last_splitter.unwrap()).unwrap();
			continue;
		}

		if input.get(down) == &Cell::Splitter {
			let v = match tail.last_splitter {
				Some(last_splitter) => *splitters.get(&last_splitter).unwrap_or(&1),
				None => 1,
			};

			match splitters.entry(down) {
				Entry::Occupied(mut entry) => {
					// Just like part 1, on the real input, there are too many differents paths,
					// so we need to skip if we already went there to save time, but still add
					// the visited value
					*entry.get_mut() += v;
					continue;
				}
				Entry::Vacant(entry) => {
					entry.insert(v);
				}
			}

			let left = down + Vector::WEST;
			if input.in_bounds(left) {
				tails.push_back(Tail {
					point: left,
					last_splitter: Some(down),
				});
			}

			let right = down + Vector::EAST;
			if input.in_bounds(right) {
				tails.push_back(Tail {
					point: right,
					last_splitter: Some(down),
				});
			}
		} else {
			tails.push_back(Tail {
				point: down,
				last_splitter: tail.last_splitter,
			});
		}
	}

	timelines
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 21);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 40);
	}
}
