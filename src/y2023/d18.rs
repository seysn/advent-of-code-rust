use regex::Regex;

enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn delta(&self, distance: isize) -> (isize, isize) {
		match self {
			Direction::Up => (0, -distance),
			Direction::Down => (0, distance),
			Direction::Left => (-distance, 0),
			Direction::Right => (distance, 0),
		}
	}
}

impl From<&str> for Direction {
	fn from(value: &str) -> Self {
		match value {
			"U" | "3" => Self::Up,
			"D" | "1" => Self::Down,
			"L" | "2" => Self::Left,
			"R" | "0" => Self::Right,
			_ => unreachable!(),
		}
	}
}

pub struct Line {
	direction: Direction,
	distance: isize,
	color: String,
}

pub fn parse_input(input: &str) -> Vec<Line> {
	let re = Regex::new(r"(U|D|L|R) (\d+) \(#(\w{6})\)").unwrap();
	input
		.lines()
		.map(|l| {
			let cap = re.captures(l).unwrap();
			Line {
				direction: cap[1].into(),
				distance: cap[2].parse().unwrap(),
				color: cap[3].to_string(),
			}
		})
		.collect()
}

#[derive(Clone, Copy)]
struct Point(isize, isize);

impl Point {
	fn add(&self, direction: &Direction, distance: isize) -> Point {
		let (x, y) = direction.delta(distance);
		Point(self.0 + x, self.1 + y)
	}
}

fn area(points: &[Point]) -> isize {
	let mut sum1: isize = 0;
	let mut sum2: isize = 0;

	for i in 1..points.len() {
		sum1 += points[i - 1].0 * points[i].1;
		sum2 += points[i - 1].1 * points[i].0;
	}
	sum1 += points[points.len() - 1].0 * points[0].1;
	sum2 += points[points.len() - 1].1 * points[0].0;

	(sum1 - sum2).abs() / 2
}

pub fn part1(input: &[Line]) -> isize {
	let mut points = Vec::new();
	let mut current = Point(0, 0);
	let mut distance = 0;

	for line in input {
		points.push(current);
		current = current.add(&line.direction, line.distance);
		distance += line.distance;
	}

	area(&points) - (distance / 2) + 1 + distance
}

pub fn part2(input: &[Line]) -> isize {
	let mut points = Vec::new();
	let mut current = Point(0, 0);
	let mut total_distance = 0;

	for line in input {
		let distance = isize::from_str_radix(&line.color[0..5], 16).unwrap();
		let direction = line.color[5..].into();
		points.push(current);
		current = current.add(&direction, distance);
		total_distance += distance;
	}

	area(&points) - (total_distance / 2) + 1 + total_distance
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 62);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 952408144115);
	}
}
