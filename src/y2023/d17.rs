use std::{
	collections::{BinaryHeap, HashMap},
	ops::Deref,
};

use crate::collections::{Grid, Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
	point: Point,
	direction: Vector,
	consecutive: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
	cost: u32,
	node: Node,
}

#[derive(Clone, Copy)]
pub struct Number(u32);

impl From<char> for Number {
	fn from(value: char) -> Self {
		Number(value.to_digit(10).unwrap())
	}
}

impl Deref for Number {
	type Target = u32;
	fn deref(&self) -> &Self::Target {
		&self.0
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

impl Grid<Number> {
	fn out_of_bounds(&self, point: Point, direction: Vector) -> bool {
		!self.in_bounds(point + direction)
	}

	fn neighbors(&self, node: &Node, min_consecutive: usize, max_consecutive: usize) -> Vec<Node> {
		let mut res = Vec::new();
		for direction in [Vector::NORTH, Vector::SOUTH, Vector::WEST, Vector::EAST] {
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

pub fn parse_input(input: &str) -> Grid<Number> {
	Grid::new(input)
}

fn search(input: &Grid<Number>, min_consecutive: usize, max_consecutive: usize) -> u32 {
	let start = Point(0, 0);
	let mut frontier: BinaryHeap<State> = BinaryHeap::new();
	frontier.push(State {
		cost: 0,
		node: Node {
			point: start,
			direction: Vector::EAST,
			consecutive: 0,
		},
	});

	let mut distances: HashMap<Node, u32> = HashMap::new();

	let goal = Point(input.width as i32 - 1, input.height as i32 - 1);
	while let Some(current) = frontier.pop() {
		if current.node.point == goal {
			return current.cost;
		}

		for next in input.neighbors(&current.node, min_consecutive, max_consecutive) {
			let new_cost = current.cost + *input.get(next.point);
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

pub fn part1(input: &Grid<Number>) -> u32 {
	search(input, 1, 3)
}

pub fn part2(input: &Grid<Number>) -> u32 {
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
