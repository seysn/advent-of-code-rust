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

fn solve(input: &[Robot], width: i32, height: i32) -> u32 {
	let mut robots = input.to_vec();
	for _ in 0..100 {
		for robot in &mut robots {
			robot.step(width, height);
		}
	}

	let mut quadrants = [0, 0, 0, 0];
	let middle_w = width / 2;
	let middle_h = height / 2;
	for robot in &robots {
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

pub fn part1(input: &[Robot]) -> u32 {
	solve(input, 101, 103)
}

pub fn part2(input: &[Robot]) -> i32 {
	let mut robots = input.to_vec();
	let n = robots.len();
	let mut xs = Vec::with_capacity(n);
	let mut ys = Vec::with_capacity(n);
	for i in 1..10000 {
		xs.clear();
		ys.clear();

		for robot in &mut robots {
			robot.step(101, 103);

			xs.push(robot.position.0);
			ys.push(robot.position.1);
		}

		let x_sum = xs.iter().sum::<i32>();
		let x_avg = x_sum / n as i32;
		let x_var = xs.iter().map(|x| (x - x_avg).pow(2)).sum::<i32>() / n as i32;

		let y_sum = ys.iter().sum::<i32>();
		let y_avg = y_sum / n as i32;
		let y_var = ys.iter().map(|y| (y - y_avg).pow(2)).sum::<i32>() / n as i32;

		if x_var < 500 && y_var < 500 {
			return i;
		}
	}

	0
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
