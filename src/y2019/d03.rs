use std::collections::{HashMap, HashSet};

use crate::collections::{Direction, Point};

type Step = (Direction, i32);

pub struct Path {
	path: Vec<Step>,
}

impl From<&str> for Path {
	fn from(value: &str) -> Self {
		Self {
			path: value
				.split(',')
				.map(|value| (Direction::from(value.chars().next().unwrap()), value[1..].parse().unwrap()))
				.collect(),
		}
	}
}

impl Path {
	fn points(&self) -> HashMap<Point, usize> {
		let mut res: HashMap<Point, usize> = HashMap::new();
		let mut point = Point(0, 0);
		let mut idx = 1;

		for (step, n) in &self.path {
			for _ in 0..*n {
				point += step;
				res.entry(point).or_insert(idx);
				idx += 1;
			}
		}

		res
	}
}

pub fn parse_input(input: &str) -> (Path, Path) {
	let mut lines = input.lines();
	let fst = Path::from(lines.next().unwrap());
	let snd = Path::from(lines.next().unwrap());
	(fst, snd)
}

pub fn part1(input: &(Path, Path)) -> i32 {
	input
		.0
		.points()
		.keys()
		.collect::<HashSet<_>>()
		.intersection(&input.1.points().keys().collect())
		.map(|p| p.0.abs() + p.1.abs())
		.min()
		.unwrap()
}

pub fn part2(input: &(Path, Path)) -> usize {
	let m0 = input.0.points();
	let m1 = input.1.points();

	m0.keys()
		.collect::<HashSet<_>>()
		.intersection(&m1.keys().collect())
		.map(|p| m0.get(p).unwrap() + m1.get(p).unwrap())
		.min()
		.unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
	const EXAMPLE2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
	const EXAMPLE3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 6);
		assert_eq!(part1(&parse_input(EXAMPLE2)), 159);
		assert_eq!(part1(&parse_input(EXAMPLE3)), 135);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 30);
		assert_eq!(part2(&parse_input(EXAMPLE2)), 610);
		assert_eq!(part2(&parse_input(EXAMPLE3)), 410);
	}
}
