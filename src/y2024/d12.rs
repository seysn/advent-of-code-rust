use std::collections::{HashSet, VecDeque};

use crate::collections::{Grid, Point, Vector};

pub fn parse_input(input: &str) -> Grid<char> {
	Grid::new(input)
}

#[derive(Debug)]
struct Region(Vec<Point>);

impl Region {
	fn new(start: Point, input: &Grid<char>) -> Self {
		let ch = input.get(start);
		let mut res = vec![start];
		let mut queue = VecDeque::new();
		queue.push_back(start);

		while let Some(p) = queue.pop_front() {
			for v in Vector::CARDINAL {
				let pp = p + v;
				if !res.contains(&pp) && input.in_bounds(pp) && input.get(pp) == ch {
					res.push(pp);
					queue.push_back(pp);
				}
			}
		}

		Self(res)
	}

	fn area(&self) -> u32 {
		self.0.len() as u32
	}

	fn perimeter(&self) -> u32 {
		let mut res = 0;
		for p in &self.0 {
			for v in Vector::CARDINAL {
				let pp = p + v;
				if !self.0.contains(&pp) {
					res += 1;
				}
			}
		}
		res
	}

	fn sides(&self) -> u32 {
		let mut visited = HashSet::new();
		let mut res = 0;
		for p in &self.0 {
			for v in Vector::CARDINAL {
				let pp = p + v;
				if visited.contains(&(*p, v)) || self.0.contains(&pp) {
					continue;
				}

				if v == Vector::NORTH || v == Vector::SOUTH {
					let mut pp = *p;
					while self.0.contains(&pp) && !self.0.contains(&(pp + v)) {
						pp += Vector::WEST;
						visited.insert((pp, v));
					}

					let mut pp = *p;
					while self.0.contains(&pp) && !self.0.contains(&(pp + v)) {
						pp += Vector::EAST;
						visited.insert((pp, v));
					}
				}
				if v == Vector::WEST || v == Vector::EAST {
					let mut pp = *p;
					while self.0.contains(&pp) && !self.0.contains(&(pp + v)) {
						pp += Vector::NORTH;
						visited.insert((pp, v));
					}

					let mut pp = *p;
					while self.0.contains(&pp) && !self.0.contains(&(pp + v)) {
						pp += Vector::SOUTH;
						visited.insert((pp, v));
					}
				}

				res += 1;
			}
		}
		res
	}
}

pub fn part1(input: &Grid<char>) -> u32 {
	let mut visited: HashSet<Point> = HashSet::new();
	let mut regions = Vec::new();
	for y in 0..input.height {
		for x in 0..input.width {
			let p = Point(x as i32, y as i32);
			if visited.contains(&p) {
				continue;
			}

			let region = Region::new(p, input);
			visited.extend(&region.0);
			regions.push(region);
		}
	}

	regions.iter().map(|r| r.area() * r.perimeter()).sum()
}

pub fn part2(input: &Grid<char>) -> u32 {
	let mut visited: HashSet<Point> = HashSet::new();
	let mut regions = Vec::new();
	for y in 0..input.height {
		for x in 0..input.width {
			let p = Point(x as i32, y as i32);
			if visited.contains(&p) {
				continue;
			}

			let region = Region::new(p, input);
			visited.extend(&region.0);
			regions.push(region);
		}
	}

	regions.iter().map(|r| r.area() * r.sides()).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 1930);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 1206);
	}
}
