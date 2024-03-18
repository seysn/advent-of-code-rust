use std::collections::HashMap;

use crate::collections::{Grid, Point};

#[derive(Debug, Copy, Clone)]
pub enum Cell {
	Start,
	End,
	Square(usize),
}

impl Cell {
	fn elevation(&self) -> usize {
		match self {
			Cell::Start => 0,
			Cell::End => 25,
			Cell::Square(i) => *i,
		}
	}
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
		match value {
			'S' => Self::Start,
			'E' => Self::End,
			c => Self::Square(c as usize - 97),
		}
	}
}

impl Grid<Cell> {
	fn start(&self) -> (usize, usize) {
		for (i, cell) in self.cells.iter().enumerate() {
			if let Cell::Start = cell {
				return (i % self.width, i / self.width);
			}
		}

		unreachable!()
	}

	fn end(&self) -> (usize, usize) {
		for (i, cell) in self.cells.iter().enumerate() {
			if let Cell::End = cell {
				return (i % self.width, i / self.width);
			}
		}

		unreachable!()
	}

	fn directions(&self, x: usize, y: usize) -> Vec<(i32, i32)> {
		let mut res = vec![];
		for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
			let xx = x as i32 + i;
			let yy = y as i32 + j;
			if xx < 0 || yy < 0 || xx >= self.width as i32 || yy >= self.height as i32 {
				continue;
			}

			if self.get(Point(x as i32, y as i32)).elevation() + 1 >= self.get(Point(xx, yy)).elevation() {
				res.push((i, j));
			}
		}

		res
	}

	fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
		self.directions(x, y)
			.iter()
			.map(|(i, j)| ((x as i32 + i) as usize, (y as i32 + j) as usize))
			.collect()
	}

	fn distance(came_from: &HashMap<(usize, usize), (usize, usize)>, start: (usize, usize), end: (usize, usize)) -> usize {
		if !came_from.contains_key(&end) {
			// No solution, return the worst case
			return usize::MAX;
		}

		let mut res = 0;
		let mut current = end;
		while current != start {
			res += 1;
			current = *came_from.get(&current).unwrap();
		}
		res
	}

	fn generate_graph(&self, start: (usize, usize), end: Option<(usize, usize)>) -> HashMap<(usize, usize), (usize, usize)> {
		let mut frontier = vec![start];
		let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
		let mut cost_so_far: HashMap<(usize, usize), usize> = HashMap::new();
		cost_so_far.insert(start, 0);

		while let Some(current) = frontier.pop() {
			if let Some(pos) = end {
				if current == pos {
					return came_from;
				}
			}

			for next in self.neighbors(current.0, current.1) {
				let new_cost = cost_so_far.get(&current).unwrap() + 1;
				if !came_from.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
					cost_so_far.insert(next, new_cost);
					frontier.insert(0, next);
					came_from.insert(next, current);
				}
			}
		}

		came_from
	}

	fn shortest_path(&self, start: (usize, usize), end: (usize, usize)) -> usize {
		let graph = self.generate_graph(start, Some(end));
		Grid::distance(&graph, start, end)
	}

	fn best_distance(&self, start: (usize, usize)) -> usize {
		let graph = self.generate_graph(start, None);

		self.cells
			.iter()
			.enumerate()
			.filter(|(_, c)| matches!(c, Cell::Square(25)))
			.map(|(i, _)| (i % self.width, i / self.width))
			.map(|end| Grid::distance(&graph, start, end))
			.min()
			.unwrap()
	}
}

pub fn parse_input(input: &str) -> Grid<Cell> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Cell>) -> usize {
	input.shortest_path(input.start(), input.end())
}

pub fn part2(input: &Grid<Cell>) -> usize {
	let reversed: Vec<Cell> = input
		.cells
		.iter()
		.map(|c| match c {
			Cell::Start => Cell::End,
			Cell::End => Cell::Start,
			Cell::Square(x) => Cell::Square(25 - x),
		})
		.collect();

	let reversed_grid = Grid {
		cells: reversed,
		height: input.height,
		width: input.width,
	};

	reversed_grid.best_distance(reversed_grid.start())
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 31);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 29);
	}
}
