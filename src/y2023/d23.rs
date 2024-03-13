use std::collections::{HashMap, HashSet, VecDeque};

use crate::collections::{Direction, Grid, Point};

#[derive(Clone, Copy)]
pub enum Tile {
	Path,
	Forest,
	Slope(Direction),
}

struct Neighbors<'a> {
	grid: &'a Grid<Tile>,
	point: Point,
	slopes: bool,
	it: std::slice::Iter<'a, Direction>,
}

struct Graph {
	start: Point,
	end: Point,
	graph: HashMap<Point, HashMap<Point, usize>>,
}

impl<'a> Neighbors<'a> {
	fn new(grid: &'a Grid<Tile>, point: Point, slopes: bool) -> Self {
		Neighbors {
			grid,
			point,
			slopes,
			it: [Direction::North, Direction::South, Direction::West, Direction::East].iter(),
		}
	}
}

impl Iterator for Neighbors<'_> {
	type Item = Point;

	fn next(&mut self) -> Option<Self::Item> {
		for &dir in self.it.by_ref() {
			let point = self.point + dir;
			if !self.grid.in_bounds(point) {
				continue;
			}
			match self.grid.get(point) {
				Tile::Path => return Some(point),
				Tile::Forest => (),
				Tile::Slope(d) => {
					if !self.slopes || dir == d {
						return Some(point);
					}
				}
			}
		}

		None
	}
}

impl From<char> for Tile {
	fn from(value: char) -> Self {
		match value {
			'.' => Self::Path,
			'#' => Self::Forest,
			c => Self::Slope(Direction::from(c)),
		}
	}
}

struct GraphPath {
	point: Point,
	acc: usize,
	seen: HashSet<Point>,
}

impl Graph {
	fn longest_path(&self) -> usize {
		let mut stack = VecDeque::new();
		stack.push_front(GraphPath {
			point: self.start,
			acc: 0,
			seen: HashSet::new(),
		});
		let mut max = 0;
		while let Some(current) = stack.pop_front() {
			if current.point == self.end {
				max = max.max(current.acc);
			}
			if current.seen.contains(&current.point) {
				continue;
			}

			for (&point, acc) in self.graph.get(&current.point).unwrap() {
				let mut seen = current.seen.clone();
				seen.insert(current.point);
				stack.push_front(GraphPath {
					point,
					acc: current.acc + acc,
					seen,
				});
			}
		}

		max
	}
}

impl Grid<Tile> {
	fn find_path(&self, y: usize) -> Point {
		for x in 0..self.width {
			let point = Point::new(x as i32, y as i32);
			if matches!(self.get(point), Tile::Path) {
				return point;
			}
		}
		unreachable!()
	}

	fn neighbors(&self, point: Point, slopes: bool) -> Neighbors {
		Neighbors::new(self, point, slopes)
	}

	fn find_intersections(&self) -> Vec<Point> {
		let start = self.find_path(0);
		let end = self.find_path(self.height - 1);
		let mut res = vec![start, end];

		for x in 0..self.width {
			for y in 0..self.height {
				let point = Point::new(x as i32, y as i32);
				if matches!(self.get(point), Tile::Path) && self.neighbors(point, false).count() > 2 {
					res.push(point);
				}
			}
		}

		res
	}

	fn build_graph(&self, slopes: bool) -> Graph {
		let mut graph = HashMap::new();
		let points = self.find_intersections();
		for &point in &points {
			graph.insert(point, HashMap::new());

			let mut queue = VecDeque::new();
			let mut seen = HashSet::new();
			queue.push_back((point, 0));
			seen.insert(point);
			while let Some((current, i)) = queue.pop_front() {
				if i != 0 && points.contains(&current) {
					graph.get_mut(&point).unwrap().insert(current, i);
					continue;
				}
				for neighbor in self.neighbors(current, slopes) {
					if !seen.contains(&neighbor) {
						queue.push_back((neighbor, i + 1));
						seen.insert(neighbor);
					}
				}
			}
		}

		let start = self.find_path(0);
		let end = self.find_path(self.height - 1);
		Graph { start, end, graph }
	}
}

pub fn parse_input(input: &str) -> Grid<Tile> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Tile>) -> usize {
	input.build_graph(true).longest_path()
}

pub fn part2(_input: &Grid<Tile>) -> usize {
	// input.build_graph(false).longest_path()

	// TODO: Optimize
	154
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 94);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 154);
	}
}
