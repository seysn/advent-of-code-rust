use std::{
	collections::{HashSet, VecDeque},
	fmt::{Display, Write},
};

use crate::collections::{Grid, Point, Vector};

pub fn parse_input(input: &str) -> Vec<Point> {
	input
		.lines()
		.map(|l| {
			let (x, y) = l.split_once(',').unwrap();
			Point(x.parse().unwrap(), y.parse().unwrap())
		})
		.collect()
}

#[derive(Clone)]
enum Byte {
	Safe,
	Corrupted,
}

impl Display for Byte {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Byte::Safe => f.write_char('.'),
			Byte::Corrupted => f.write_char('#'),
		}
	}
}

fn solve(input: &[Point], width: usize, height: usize) -> usize {
	let mut grid = Grid::fill(&Byte::Safe, width, height);
	for point in input {
		grid.set(*point, Byte::Corrupted);
	}

	let mut queue = VecDeque::new();
	let mut seen = HashSet::new();

	let start = Point(0, 0);
	let end = Point(width as i32 - 1, height as i32 - 1);

	queue.push_back((start, 0));
	while let Some((current, step)) = queue.pop_front() {
		if seen.contains(&current) {
			continue;
		}
		seen.insert(current);

		if current == end {
			return step;
		}

		for v in Vector::CARDINAL {
			let next = current + v;
			if !grid.in_bounds(next) || matches!(grid.get(next), &Byte::Corrupted) {
				continue;
			}
			queue.push_back((next, step + 1));
		}
	}

	0
}

pub fn part1(input: &[Point]) -> usize {
	solve(&input[..1024], 71, 71)
}

fn is_solvable(grid: &Grid<Byte>) -> bool {
	let mut queue = VecDeque::new();
	let mut seen = HashSet::new();

	let start = Point(0, 0);
	let end = Point(grid.width as i32 - 1, grid.height as i32 - 1);

	queue.push_back(start);
	while let Some(current) = queue.pop_front() {
		if seen.contains(&current) {
			continue;
		}
		seen.insert(current);

		if current == end {
			return true;
		}

		for v in Vector::CARDINAL {
			let next = current + v;
			if !grid.in_bounds(next) || matches!(grid.get(next), &Byte::Corrupted) {
				continue;
			}
			queue.push_back(next);
		}
	}

	false
}

fn solve2(input: &[Point], init: usize, width: usize, height: usize) -> String {
	let mut grid = Grid::fill(&Byte::Safe, width, height);
	for point in &input[..init] {
		grid.set(*point, Byte::Corrupted);
	}

	for point in &input[init..] {
		grid.set(*point, Byte::Corrupted);
		if !is_solvable(&grid) {
			return format!("{},{}", point.0, point.1);
		}
	}

	"0,0".to_owned()
}

pub fn part2(input: &[Point]) -> String {
	// TODO: Optimize
	solve2(input, 1024, 71, 71)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

	#[test]
	fn example_part1() {
		assert_eq!(solve(&parse_input(EXAMPLE)[..12], 7, 7), 22);
	}

	#[test]
	fn example_part2() {
		assert_eq!(&solve2(&parse_input(EXAMPLE), 12, 7, 7), "6,1");
	}
}
