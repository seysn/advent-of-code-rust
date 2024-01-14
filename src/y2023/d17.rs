use std::{
	collections::{BinaryHeap, HashMap},
	ops::Add,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	North,
	South,
	West,
	East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
	x: usize,
	y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
	point: Point,
	direction: Direction,
	consecutive: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
	cost: u32,
	node: Node,
}

pub struct Grid {
	cells: Vec<u32>,
	width: usize,
	height: usize,
}

impl Direction {
	fn reverse(&self) -> Direction {
		match self {
			Direction::North => Direction::South,
			Direction::South => Direction::North,
			Direction::West => Direction::East,
			Direction::East => Direction::West,
		}
	}

	fn delta(&self) -> (i32, i32) {
		match self {
			Direction::North => (0, -1),
			Direction::South => (0, 1),
			Direction::West => (-1, 0),
			Direction::East => (1, 0),
		}
	}
}

impl Point {
	fn new(x: usize, y: usize) -> Self {
		Self { x, y }
	}
}

impl Add<Direction> for Point {
	type Output = Point;

	fn add(self, direction: Direction) -> Self::Output {
		let (x, y) = direction.delta();
		Point {
			x: (self.x as i32 + x) as usize,
			y: (self.y as i32 + y) as usize,
		}
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		// Implementation is here just for the BinaryHeap
		other.cost.cmp(&self.cost)
	}
}

impl Grid {
	fn get(&self, point: Point) -> u32 {
		self.cells[self.width * point.y + point.x]
	}

	fn out_of_bounds(&self, point: Point, direction: Direction) -> bool {
		match direction {
			Direction::North => point.y == 0,
			Direction::South => point.y == self.height - 1,
			Direction::West => point.x == 0,
			Direction::East => point.x == self.width - 1,
		}
	}

	fn neighbors(&self, node: &Node, min_consecutive: usize, max_consecutive: usize) -> Vec<Node> {
		let mut res = Vec::new();
		for direction in [Direction::North, Direction::South, Direction::West, Direction::East] {
			if self.out_of_bounds(node.point, direction) {
				continue;
			}

			if direction == node.direction.reverse() {
				continue;
			} else if direction != node.direction && node.consecutive >= min_consecutive {
				res.push(Node {
					point: node.point + direction,
					direction,
					consecutive: 1,
				});
			} else if direction == node.direction && node.consecutive < max_consecutive {
				res.push(Node {
					point: node.point + direction,
					direction,
					consecutive: node.consecutive + 1,
				});
			}
		}

		res
	}
}

pub fn parse_input(input: &str) -> Grid {
	let cells: Vec<u32> = input
		.chars()
		.filter(|c| !c.is_whitespace())
		.map(|c| c.to_digit(10).unwrap())
		.collect();
	let width = input.lines().next().unwrap().len();
	let height = cells.len() / width;

	Grid { cells, width, height }
}

fn search(input: &Grid, min_consecutive: usize, max_consecutive: usize) -> u32 {
	let start = Point::new(0, 0);
	let mut frontier: BinaryHeap<State> = BinaryHeap::new();
	frontier.push(State {
		cost: 0,
		node: Node {
			point: start,
			direction: Direction::East,
			consecutive: 0,
		},
	});

	let mut distances: HashMap<Node, u32> = HashMap::new();

	let goal = Point::new(input.width - 1, input.height - 1);
	while let Some(current) = frontier.pop() {
		if current.node.point == goal {
			return current.cost;
		}

		for next in input.neighbors(&current.node, min_consecutive, max_consecutive) {
			let new_cost = current.cost + input.get(next.point);
			if let Some(&best) = distances.get(&next) {
				if new_cost >= best {
					continue;
				}
			}

			distances.insert(next, new_cost);
			frontier.push(State {
				cost: new_cost,
				node: next,
			});
		}
	}

	0
}

pub fn part1(input: &Grid) -> u32 {
	search(input, 1, 3)
}

pub fn part2(input: &Grid) -> u32 {
	search(input, 4, 10)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 102);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 94);
	}
}
