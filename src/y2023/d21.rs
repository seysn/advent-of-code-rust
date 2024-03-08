use std::collections::{HashSet, VecDeque};

use crate::collections::{Direction, Grid, Point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
	Plot,
	Rock,
	Position,
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
		match value {
			'.' => Cell::Plot,
			'#' => Cell::Rock,
			'S' => Cell::Position,
			_ => unreachable!(),
		}
	}
}

impl Grid<Cell> {
	fn neighbors(&self, point: Point) -> Vec<Point> {
		[Direction::North, Direction::South, Direction::West, Direction::East]
			.iter()
			.map(|&dir| point + dir)
			.filter(|&p| self.in_bounds(p) && matches!(self.get(p), Cell::Plot | Cell::Position))
			.collect()
	}

	fn steps(&self, start: &Point, n_steps: usize) -> usize {
		let mut res: HashSet<Point> = HashSet::new();
		let mut seen: HashSet<Point> = HashSet::new();
		let mut queue = VecDeque::new();
		queue.push_back((start.to_owned(), n_steps));
		while let Some((p, s)) = queue.pop_front() {
			if s % 2 == 0 {
				res.insert(p);
			}
			if s == 0 {
				continue;
			}
			for n in self.neighbors(p) {
				if seen.contains(&n) {
					continue;
				}
				seen.insert(n);
				queue.push_back((n, s - 1));
			}
		}
		res.len()
	}
}

pub fn parse_input(input: &str) -> Grid<Cell> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Cell>) -> usize {
	input.steps(input.find(Cell::Position).first().unwrap(), 64)
}

pub fn part2(input: &Grid<Cell>) -> usize {
	let steps: usize = 26501365;
	let grid_size = input.width;
	let n_grid_radius = steps.div_euclid(input.width) - 1;
	let starting_points = input.find(Cell::Position);
	let start = starting_points.first().unwrap();

	let n_grid_odd = (n_grid_radius.div_euclid(2) * 2 + 1).pow(2);
	let n_grid_even = ((n_grid_radius + 1).div_euclid(2) * 2).pow(2);

	// Number of plots for each type of grid
	let grid_odd = input.steps(start, grid_size * 2 + 1);
	let grid_even = input.steps(start, grid_size * 2);
	let grid_top = input.steps(&Point::new(start.x, grid_size as i32 - 1), grid_size - 1);
	let grid_bot = input.steps(&Point::new(start.x, 0), grid_size - 1);
	let grid_left = input.steps(&Point::new(grid_size as i32 - 1, start.y), grid_size - 1);
	let grid_right = input.steps(&Point::new(0, start.y), grid_size - 1);
	let grid_small_top_right = input.steps(&Point::new(grid_size as i32 - 1, 0), grid_size.div_euclid(2) - 1);
	let grid_small_top_left = input.steps(&Point::new(grid_size as i32 - 1, grid_size as i32 - 1), grid_size.div_euclid(2) - 1);
	let grid_small_bot_right = input.steps(&Point::new(0, 0), grid_size.div_euclid(2) - 1);
	let grid_small_bot_left = input.steps(&Point::new(0, grid_size as i32 - 1), grid_size.div_euclid(2) - 1);
	let grid_large_top_right = input.steps(&Point::new(grid_size as i32 - 1, 0), (grid_size * 3).div_euclid(2) - 1);
	let grid_large_top_left = input.steps(
		&Point::new(grid_size as i32 - 1, grid_size as i32 - 1),
		(grid_size * 3).div_euclid(2) - 1,
	);
	let grid_large_bot_right = input.steps(&Point::new(0, 0), (grid_size * 3).div_euclid(2) - 1);
	let grid_large_bot_left = input.steps(&Point::new(0, grid_size as i32 - 1), (grid_size * 3).div_euclid(2) - 1);

	(n_grid_odd * grid_odd)
		+ (n_grid_even * grid_even)
		+ (grid_top + grid_bot + grid_left + grid_right)
		+ (n_grid_radius + 1) * (grid_small_bot_left + grid_small_bot_right + grid_small_top_left + grid_small_top_right)
		+ n_grid_radius * (grid_large_bot_left + grid_large_bot_right + grid_large_top_left + grid_large_top_right)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

	#[test]
	fn test_neighbors() {
		let grid = parse_input(EXAMPLE);
		let n = grid.neighbors(Point::new(7, 0));
		assert!(n.contains(&Point::new(6, 0)));
		assert!(n.contains(&Point::new(8, 0)));
		assert!(!n.contains(&Point::new(7, 1)));
		assert!(!n.contains(&Point::new(7, -1)));
	}

	#[test]
	fn example_part1() {
		let grid = parse_input(EXAMPLE);
		let starting_points = grid.find(Cell::Position);
		let start = starting_points.first().unwrap();
		assert_eq!(grid.steps(start, 1), 2);
		assert_eq!(grid.steps(start, 2), 4);
		assert_eq!(grid.steps(start, 6), 16);
	}

	#[test]
	fn example_part2() {
		// assert_eq!(parse_input(EXAMPLE).steps_wrapped(6), 16);
		// assert_eq!(parse_input(EXAMPLE).steps_wrapped(10), 50);
		// assert_eq!(parse_input(EXAMPLE).steps_wrapped(50), 1594);
		// assert_eq!(parse_input(EXAMPLE).steps_wrapped(100), 6536);
		// assert_eq!(parse_input(EXAMPLE).steps_wrapped(500), 167004);
		// assert_eq!(parse_input(EXAMPLE).steps_wrapped(1000), 668697);
		// assert_eq!(parse_input(EXAMPLE).steps_wrapped(5000), 16733044);
	}
}
