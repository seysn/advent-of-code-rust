use std::collections::HashSet;

use num::integer::gcd;

use crate::collections::Point;

#[derive(Debug, Clone)]
pub struct Map {
	asteroids: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Delta(i32, i32);

impl Map {
	fn best_position(&self) -> (&Point, HashSet<Delta>) {
		self.asteroids
			.iter()
			.map(|a| (a, self.find_deltas(a)))
			.max_by(|a, b| a.1.len().cmp(&b.1.len()))
			.unwrap_or((&Point(0, 0), HashSet::new()))
	}

	fn find_deltas(&self, base: &Point) -> HashSet<Delta> {
		let mut res = HashSet::new();

		for p in &self.asteroids {
			if p == base {
				continue;
			}

			res.insert(Delta::from_point(p, base).reduce());
		}

		res
	}

	fn vaporized(&self, base: &Point, nth: usize) -> Point {
		let mut asteroids: Vec<Point> = self.asteroids.iter().filter(|&p| p != base).cloned().collect();
		let deltas = self.find_deltas(base);

		let mut order: Vec<(&Delta, f32)> = deltas.iter().map(|d| (d, d.angle())).collect();
		order.sort_by(|a, b| a.1.total_cmp(&b.1));

		let mut idx = 0;
		for (delta, _) in order.iter().cycle() {
			if let Some((p, _)) = asteroids
				.iter()
				.filter_map(|p| {
					let d = Delta::from_point(p, base);
					if d.reduce() == **delta {
						Some((*p, d))
					} else {
						None
					}
				})
				.min_by(|a, b| a.1.distance().total_cmp(&b.1.distance()))
			{
				idx += 1;
				if idx == nth {
					return p;
				}
				asteroids.retain(|&x| x != p);
			}
		}

		unreachable!()
	}
}

impl Delta {
	fn from_point(p: &Point, base: &Point) -> Self {
		Self(p.0 - base.0, p.1 - base.1)
	}

	fn reduce(&self) -> Self {
		let gcd = gcd(self.0, self.1);
		Self(self.0 / gcd, self.1 / gcd)
	}

	fn distance(&self) -> f32 {
		(self.0 as f32 * self.0 as f32 + self.1 as f32 * self.1 as f32).sqrt()
	}

	fn angle(&self) -> f32 {
		let a = Self(0, -1);

		let dot_product = a.0 as f32 * self.0 as f32 + a.1 as f32 * self.1 as f32;
		let magnitude_b = self.distance();

		let res = (dot_product / magnitude_b).acos();

		if self.0 < 0 {
			// Not sure mathematicians would like this but it works
			10.0 - res
		} else {
			res
		}
	}
}

pub fn parse_input(input: &str) -> Map {
	let mut asteroids = Vec::new();
	for (j, l) in input.lines().enumerate() {
		for (i, c) in l.chars().enumerate() {
			if c == '#' {
				asteroids.push(Point(i as i32, j as i32));
			}
		}
	}
	Map { asteroids }
}

pub fn part1(input: &Map) -> usize {
	input.best_position().1.len()
}

pub fn part2(input: &Map) -> i32 {
	let base = input.best_position().0;
	let res = input.vaporized(base, 200);
	res.0 * 100 + res.1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		let example = parse_input(".#..#\n.....\n#####\n....#\n...##");
		assert_eq!(example.find_deltas(&Point(1, 0)).len(), 7);
		assert_eq!(example.find_deltas(&Point(0, 2)).len(), 6);
		assert_eq!(example.find_deltas(&Point(4, 2)).len(), 5);
		assert_eq!(example.find_deltas(&Point(3, 4)).len(), 8);
		assert_eq!(part1(&example), 8);
	}

	#[test]
	fn example_part2() {
		let example = parse_input(".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##");
		assert_eq!(example.vaporized(&Point(8, 3), 1), Point(8, 1));
		assert_eq!(example.vaporized(&Point(8, 3), 2), Point(9, 0));
		assert_eq!(example.vaporized(&Point(8, 3), 3), Point(9, 1));
		assert_eq!(example.vaporized(&Point(8, 3), 9), Point(15, 1));
		assert_eq!(example.vaporized(&Point(8, 3), 10), Point(12, 2));
		assert_eq!(example.vaporized(&Point(8, 3), 14), Point(12, 3));
		assert_eq!(example.vaporized(&Point(8, 3), 15), Point(16, 4));
		assert_eq!(example.vaporized(&Point(8, 3), 16), Point(15, 4));
		assert_eq!(example.vaporized(&Point(8, 3), 17), Point(10, 4));
		assert_eq!(example.vaporized(&Point(8, 3), 18), Point(4, 4));
		assert_eq!(example.vaporized(&Point(8, 3), 23), Point(0, 1));
		assert_eq!(example.vaporized(&Point(8, 3), 31), Point(8, 0));
		assert_eq!(example.vaporized(&Point(8, 3), 32), Point(10, 1));
		assert_eq!(example.vaporized(&Point(8, 3), 33), Point(14, 0));
		assert_eq!(example.vaporized(&Point(8, 3), 34), Point(16, 1));
		assert_eq!(example.vaporized(&Point(8, 3), 35), Point(13, 3));
		assert_eq!(example.vaporized(&Point(8, 3), 36), Point(14, 3));

		let example = parse_input(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##");
		assert_eq!(example.vaporized(&Point(11, 13), 1), Point(11, 12));
		assert_eq!(example.vaporized(&Point(11, 13), 2), Point(12, 1));
		assert_eq!(example.vaporized(&Point(11, 13), 3), Point(12, 2));
		assert_eq!(example.vaporized(&Point(11, 13), 10), Point(12, 8));
		assert_eq!(example.vaporized(&Point(11, 13), 20), Point(16, 0));
		assert_eq!(example.vaporized(&Point(11, 13), 50), Point(16, 9));
		assert_eq!(example.vaporized(&Point(11, 13), 100), Point(10, 16));
		assert_eq!(example.vaporized(&Point(11, 13), 199), Point(9, 6));
		assert_eq!(example.vaporized(&Point(11, 13), 200), Point(8, 2));
		assert_eq!(example.vaporized(&Point(11, 13), 201), Point(10, 9));
		assert_eq!(example.vaporized(&Point(11, 13), 299), Point(11, 1));
	}
}
