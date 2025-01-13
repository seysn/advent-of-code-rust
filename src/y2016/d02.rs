use std::collections::HashMap;

use crate::collections::{Point, Vector};

pub fn parse_input(input: &str) -> Vec<String> {
	input.lines().map(|l| l.to_owned()).collect()
}

fn solve(input: &[String], keypad: HashMap<Point, char>) -> String {
	let mut code = String::new();
	let mut point = Point(0, 0);
	for l in input {
		for ch in l.chars() {
			let new = point + Vector::from(ch);
			if keypad.contains_key(&new) {
				point = new;
			}
		}

		code.push(*keypad.get(&point).unwrap());
	}

	code
}

pub fn part1(input: &[String]) -> String {
	let keypad: HashMap<Point, char> = [
		(Point(-1, -1), '1'),
		(Point(0, -1), '2'),
		(Point(1, -1), '3'),
		(Point(-1, 0), '4'),
		(Point(0, 0), '5'),
		(Point(1, 0), '6'),
		(Point(-1, 1), '7'),
		(Point(0, 1), '8'),
		(Point(1, 1), '9'),
	]
	.into_iter()
	.collect();

	solve(input, keypad)
}

pub fn part2(input: &[String]) -> String {
	let keypad: HashMap<Point, char> = [
		(Point(2, -2), '1'),
		(Point(1, -1), '2'),
		(Point(2, -1), '3'),
		(Point(3, -1), '4'),
		(Point(0, 0), '5'),
		(Point(1, 0), '6'),
		(Point(2, 0), '7'),
		(Point(3, 0), '8'),
		(Point(4, 0), '9'),
		(Point(1, 1), 'A'),
		(Point(2, 1), 'B'),
		(Point(3, 1), 'C'),
		(Point(2, 2), 'D'),
	]
	.into_iter()
	.collect();

	solve(input, keypad)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "ULL
RRDDD
LURDL
UUUUD";

	#[test]
	fn example_part1() {
		assert_eq!(&part1(&parse_input(EXAMPLE)), "1985");
	}

	#[test]
	fn example_part2() {
		assert_eq!(&part2(&parse_input(EXAMPLE)), "5DB3");
	}
}
