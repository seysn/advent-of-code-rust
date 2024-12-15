use regex::Regex;

use crate::collections::{Point, Vector};

#[derive(Debug, Clone)]
pub struct Robot {
	position: Point,
	velocity: Vector,
}

pub fn parse_input(input: &str) -> Vec<Robot> {
	let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

	input
		.lines()
		.map(|l| {
			let caps = re.captures(l).unwrap();

			Robot {
				position: Point(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
				velocity: Vector(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
			}
		})
		.collect()
}

impl Robot {
	fn step(&mut self, width: i32, height: i32) {
		let next = self.position + self.velocity;
		self.position = Point(next.0.rem_euclid(width), next.1.rem_euclid(height));
	}
}

fn safety_factor(robots: &[Robot], width: i32, height: i32) -> u32 {
	let mut quadrants = [0, 0, 0, 0];
	let middle_w = width / 2;
	let middle_h = height / 2;
	for robot in robots {
		match (robot.position.0.cmp(&middle_w), robot.position.1.cmp(&middle_h)) {
			(std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quadrants[0] += 1,
			(std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quadrants[1] += 1,
			(std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quadrants[2] += 1,
			(std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quadrants[3] += 1,
			_ => (),
		}
	}

	quadrants.iter().product()
}

fn solve(input: &[Robot], width: i32, height: i32) -> u32 {
	let mut robots = input.to_vec();
	for _ in 0..100 {
		for robot in &mut robots {
			robot.step(width, height);
		}
	}

	safety_factor(&robots, width, height)
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn part1(input: &[Robot]) -> u32 {
	solve(input, WIDTH, HEIGHT)
}

/// This solution use the fact that our tree is small and localized in one of the four
/// quadrant, so that mean the safety factor will be small in this case since the product
/// of 4 numbers is larger if these numbers are all high, and not just one.
/// The number of loops is hardcoded to 10000, I don't think there's any solution that
/// need more.
pub fn part2(input: &[Robot]) -> i32 {
	let mut robots = input.to_vec();
	let mut min_s = 0;
	let mut min_v = safety_factor(&robots, WIDTH, HEIGHT);
	for s in 1..10000 {
		for robot in &mut robots {
			robot.step(WIDTH, HEIGHT);
		}

		let v: u32 = safety_factor(&robots, WIDTH, HEIGHT);
		if v < min_v {
			min_s = s;
			min_v = v;
		}
	}

	min_s
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

	#[test]
	fn example_part1() {
		assert_eq!(solve(&parse_input(EXAMPLE), 11, 7), 12);
	}
}
