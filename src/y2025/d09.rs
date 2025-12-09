use crate::collections::Point;

pub fn parse_input(input: &str) -> Vec<Point> {
	input
		.lines()
		.map(|l| {
			let (x, y) = l.split_once(',').unwrap();
			Point(x.parse().unwrap(), y.parse().unwrap())
		})
		.collect()
}

fn area(p0: Point, p1: Point) -> u64 {
	((p0.0 - p1.0).abs() + 1) as u64 * ((p0.1 - p1.1).abs() + 1) as u64
}

pub fn part1(input: &[Point]) -> u64 {
	let mut res = 0;

	for (i, a) in input.iter().enumerate() {
		for b in &input[i + 1..] {
			res = res.max(area(*a, *b));
		}
	}

	res
}

struct Rectangle {
	min_x: i32,
	max_x: i32,
	min_y: i32,
	max_y: i32,
}

impl Rectangle {
	fn new(p0: Point, p1: Point) -> Self {
		Self {
			min_x: p0.0.min(p1.0),
			max_x: p0.0.max(p1.0),
			min_y: p0.1.min(p1.1),
			max_y: p0.1.max(p1.1),
		}
	}

	/// Search for an intersection of the rectangle with every edges
	/// If there's one intersection, it means the rectangle goes outside
	/// Edges act like thin rectangles
	fn has_intersection(&self, edges: &[(Point, Point)]) -> bool {
		for edge in edges {
			let min_x = edge.0 .0.min(edge.1 .0);
			let max_x = edge.0 .0.max(edge.1 .0);
			let min_y = edge.0 .1.min(edge.1 .1);
			let max_y = edge.0 .1.max(edge.1 .1);

			if self.min_x < max_x && self.max_x > min_x && self.min_y < max_y && self.max_y > min_y {
				return true;
			}
		}

		false
	}
}

pub fn part2(input: &[Point]) -> u64 {
	let mut edges = Vec::new();
	for ps in input.windows(2) {
		edges.push((ps[0], ps[1]));
	}
	edges.push((*input.first().unwrap(), *input.last().unwrap()));

	let mut res = 0;
	for (i, p0) in input.iter().enumerate() {
		for p1 in &input[i + 1..] {
			let area = area(*p0, *p1);
			if area > res {
				let rectangle = Rectangle::new(*p0, *p1);
				if !rectangle.has_intersection(&edges) {
					res = area;
				}
			}
		}
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 50);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 24);
	}
}
