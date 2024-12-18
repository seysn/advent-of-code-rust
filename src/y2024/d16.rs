use std::{
	cmp::Ordering,
	collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use crate::collections::{Grid, Point, Vector};

#[derive(PartialEq)]
pub enum Cell {
	Path,
	Wall,
	Start,
	End,
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
		match value {
			'#' => Self::Wall,
			'S' => Self::Start,
			'E' => Self::End,
			_ => Self::Path,
		}
	}
}

pub fn parse_input(input: &str) -> Grid<Cell> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Cell>) -> u32 {
	let start = input.find(&Cell::Start).unwrap();
	let mut seen: HashMap<(Point, Vector), u32> = HashMap::new();
	let mut queue: VecDeque<(Point, Vector, u32)> = VecDeque::new();
	queue.push_back((start, Vector::EAST, 0));
	seen.insert((start, Vector::EAST), 0);

	let mut res = u32::MAX;
	while let Some((p, v, s)) = queue.pop_front() {
		for d in Vector::CARDINAL {
			let pp = p + d;
			let cell = input.get(pp);
			if cell == &Cell::Wall {
				continue;
			}

			let mut ss = s;
			if v != d {
				let score = seen.get(&(p, d)).unwrap_or(&u32::MAX);
				if s < *score {
					ss += 1000;
					seen.insert((p, d), ss);
					queue.push_back((p, d, ss));
				} else {
					continue;
				}
			}

			ss += 1;
			let score = seen.get(&(pp, d)).unwrap_or(&u32::MAX);
			if cell == &Cell::End && ss < res {
				res = ss;
			} else if ss < *score {
				seen.insert((pp, d), ss);
				queue.push_back((pp, d, ss));
			}
		}
	}

	res
}

#[derive(PartialEq, Eq)]
struct GridPath {
	state: State,
	s: u32,
}

impl GridPath {
	fn new(state: State, s: u32) -> Self {
		Self { state, s }
	}
}
impl PartialOrd for GridPath {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for GridPath {
	fn cmp(&self, other: &Self) -> Ordering {
		self.s.cmp(&other.s).reverse()
	}
}

#[derive(PartialEq, Eq)]
struct State {
	p: Point,
	v: Vector,
	path: HashSet<Point>,
}

pub fn part2(input: &Grid<Cell>) -> usize {
	let start = input.find(&Cell::Start).unwrap();
	let mut seen: HashMap<(Point, Vector), u32> = HashMap::new();
	let mut queue: BinaryHeap<GridPath> = BinaryHeap::new();
	queue.push(GridPath::new(
		State {
			p: start,
			v: Vector::EAST,
			path: HashSet::new(),
		},
		0,
	));
	seen.insert((start, Vector::EAST), 0);

	let mut max = u32::MAX;
	let mut res: HashSet<Point> = HashSet::new();
	while let Some(path) = queue.pop() {
		let p = path.state.p;
		let v = path.state.v;
		let s = path.s;
		if s > max {
			break;
		}

		for d in Vector::CARDINAL {
			if v.reverse() == d {
				continue;
			}

			let pp = p + d;
			let cell = input.get(pp);
			if cell == &Cell::Wall {
				seen.insert((p, d), s);
				continue;
			}

			let mut ss = s;
			if v != d {
				ss += 1000;
			}

			ss += 1;
			if cell == &Cell::End {
				max = ss;
				res.extend(&path.state.path);
				continue;
			}

			let score = *seen.get(&(pp, d)).unwrap_or(&u32::MAX);
			if ss <= score {
				seen.insert((pp, d), ss);
				let mut new_path = HashSet::new();
				new_path.extend(&path.state.path);
				new_path.insert(pp);
				queue.push(GridPath::new(
					State {
						p: pp,
						v: d,
						path: new_path,
					},
					ss,
				));
			}
		}
	}

	res.len() + 2
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

	const EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 7036);
		assert_eq!(part1(&parse_input(EXAMPLE2)), 11048);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 45);
		assert_eq!(part2(&parse_input(EXAMPLE2)), 64);
	}
}
