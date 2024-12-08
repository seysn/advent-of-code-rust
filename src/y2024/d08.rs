use std::collections::{HashMap, HashSet};

use crate::collections::{Grid, Point};

pub fn parse_input(input: &str) -> Grid<char> {
	Grid::new(input)
}

fn antennas(input: &Grid<char>) -> HashMap<char, Vec<Point>> {
	let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
	for j in 0..input.height {
		for i in 0..input.width {
			let p = Point(i as i32, j as i32);
			let c = input.get(p);
			if c != '.' {
				antennas.entry(c).or_default().push(p);
			}
		}
	}
	antennas
}

pub fn part1(input: &Grid<char>) -> usize {
	let antennas = antennas(input);

	let mut antinodes: HashSet<Point> = HashSet::new();
	for ps in antennas.values() {
		for p in ps {
			for pp in ps {
				if p == pp {
					continue;
				}

				let v = p.vector(pp);
				let antinode = pp + v;
				if input.in_bounds(antinode) {
					antinodes.insert(antinode);
				}

				let v = -v;
				let antinode = p + v;
				if input.in_bounds(antinode) {
					antinodes.insert(antinode);
				}
			}
		}
	}

	antinodes.len()
}

pub fn part2(input: &Grid<char>) -> usize {
	let antennas = antennas(input);

	let mut antinodes: HashSet<Point> = HashSet::new();
	for ps in antennas.values() {
		for p in ps {
			antinodes.insert(*p);
			for pp in ps {
				if p == pp {
					continue;
				}

				let v = p.vector(pp);
				let mut antinode = pp + v;
				while input.in_bounds(antinode) {
					antinodes.insert(antinode);
					antinode += v;
				}

				let v = -v;
				let mut antinode = p + v;
				while input.in_bounds(antinode) {
					antinodes.insert(antinode);
					antinode += v;
				}
			}
		}
	}

	antinodes.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 14);
	}

	#[test]
	fn example_part2() {
		// 		assert_eq!(
		// 			part2(&parse_input(
		// 				"T.........
		// ...T......
		// .T........
		// ..........
		// ..........
		// ..........
		// ..........
		// ..........
		// ..........
		// .........."
		// 			)),
		// 			9
		// 		);
		assert_eq!(part2(&parse_input(EXAMPLE)), 34);
	}
}
