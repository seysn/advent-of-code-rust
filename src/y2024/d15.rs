use std::{
	collections::HashSet,
	fmt::{Display, Write},
};

use crate::collections::{Grid, Point, Vector};

#[derive(Clone, Copy, PartialEq)]
enum Cell {
	Void,
	Wall,
	Box,
	BoxLeft,
	BoxRight,
	Robot,
}

pub struct Input {
	grid: Grid<Cell>,
	movements: Vec<Vector>,
}

impl Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(match self {
			Cell::Void => '.',
			Cell::Wall => '#',
			Cell::Box => 'O',
			Cell::Robot => '@',
			Cell::BoxLeft => '[',
			Cell::BoxRight => ']',
		})
	}
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
		match value {
			'#' => Self::Wall,
			'O' => Self::Box,
			'@' => Self::Robot,
			_ => Self::Void,
		}
	}
}

pub fn parse_input(input: &str) -> Input {
	let (grid, movements) = input.split_once("\n\n").unwrap();

	Input {
		grid: Grid::new(grid),
		movements: movements
			.chars()
			.filter_map(|c| if c.is_whitespace() { None } else { Some(c.into()) })
			.collect(),
	}
}

fn search_void(grid: &Grid<Cell>, cell: Point, movement: Vector) -> Option<Point> {
	let mut p = cell;
	while grid[p] != Cell::Wall {
		if grid[p] == Cell::Void {
			return Some(p);
		}

		p += movement;
	}

	None
}

pub fn part1(input: &Input) -> i32 {
	let mut grid = input.grid.clone();
	let mut robot = *grid.find(Cell::Robot).first().unwrap();

	for v in &input.movements {
		let new = robot + v;
		match grid[new] {
			Cell::Void => {
				grid.swap(robot, new);
				robot = new;
			}
			Cell::Wall => (),
			Cell::Box => {
				if let Some(p) = search_void(&grid, new, *v) {
					let rv = v.reverse();
					let mut pp = p;
					while pp != robot {
						grid.swap(pp, pp + rv);
						pp += rv;
					}
					robot = new;
				}
			}
			_ => unreachable!(),
		}
	}

	grid.find(Cell::Box).iter().map(|p| 100 * p.1 + p.0).sum()
}

fn search_voids(grid: &Grid<Cell>, cell: Point, movement: Vector) -> HashSet<Point> {
	let (left, right) = if grid[cell] == Cell::BoxLeft {
		(cell, cell + Vector::EAST)
	} else {
		(cell + Vector::WEST, cell)
	};

	let left_next = left + movement;
	let mut l = match grid[left_next] {
		Cell::Void => {
			let mut res = HashSet::new();
			res.insert(left_next);
			res
		}
		Cell::Wall => HashSet::new(),
		Cell::BoxLeft | Cell::BoxRight => search_voids(grid, left_next, movement),
		_ => unreachable!(),
	};

	if l.is_empty() {
		return l;
	}

	let right_next = right + movement;
	let r = match grid[right_next] {
		Cell::Void => {
			let mut res = HashSet::new();
			res.insert(right_next);
			res
		}
		Cell::Wall => HashSet::new(),
		Cell::BoxLeft | Cell::BoxRight => search_voids(grid, right_next, movement),
		_ => unreachable!(),
	};

	if r.is_empty() {
		return r;
	}

	l.extend(&r);
	l
}

pub fn part2(input: &Input) -> i32 {
	let mut grid = Grid::fill(Cell::Void, input.grid.width * 2, input.grid.height);
	for (i, c) in input.grid.cells.iter().enumerate() {
		match c {
			Cell::Void | Cell::Wall => {
				grid.cells[i * 2] = *c;
				grid.cells[i * 2 + 1] = *c;
			}
			Cell::Box => {
				grid.cells[i * 2] = Cell::BoxLeft;
				grid.cells[i * 2 + 1] = Cell::BoxRight;
			}
			Cell::Robot => {
				grid.cells[i * 2] = Cell::Robot;
				grid.cells[i * 2 + 1] = Cell::Void;
			}
			Cell::BoxLeft | Cell::BoxRight => unreachable!(),
		}
	}

	let mut robot = *grid.find(Cell::Robot).first().unwrap();
	for v in &input.movements {
		let new = robot + v;
		match (grid[new], v) {
			(Cell::Void, _) => {
				grid.swap(robot, new);
				robot = new;
			}
			(Cell::Wall, _) => (),
			(Cell::BoxLeft | Cell::BoxRight, &Vector::WEST | &Vector::EAST) => {
				if let Some(p) = search_void(&grid, new, *v) {
					let rv = v.reverse();
					let mut pp = p;
					while pp != robot {
						grid.swap(pp, pp + rv);
						pp += rv;
					}
					robot = new;
				}
			}
			(Cell::BoxLeft | Cell::BoxRight, &Vector::NORTH | &Vector::SOUTH) => {
				let rv = v.reverse();
				loop {
					if grid[new] == Cell::Void {
						grid.swap(robot, new);
						robot = new;
						break;
					}

					let pts = search_voids(&grid, new, *v);
					if pts.is_empty() {
						break;
					}

					let y = if v == &Vector::NORTH {
						pts.iter().map(|p| p.1).min().unwrap()
					} else {
						pts.iter().map(|p| p.1).max().unwrap()
					};

					for p in pts {
						if p.1 != y {
							continue;
						}
						let pp = p + rv;
						grid.swap(p, pp);
					}
				}
			}
			_ => unreachable!(),
		}
	}

	grid.find(Cell::BoxLeft).iter().map(|p| 100 * p.1 + p.0).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 10092);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 9021);
	}
}
