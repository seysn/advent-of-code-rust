use std::collections::HashMap;

use crate::collections::{Grid, Point};

#[derive(Clone, Copy)]
pub enum Tile {
	NS,
	WE,
	NE,
	NW,
	SW,
	SE,
	G,
	S,
}

impl From<char> for Tile {
	fn from(value: char) -> Self {
		match value {
			'|' => Tile::NS,
			'-' => Tile::WE,
			'L' => Tile::NE,
			'J' => Tile::NW,
			'7' => Tile::SW,
			'F' => Tile::SE,
			'.' => Tile::G,
			'S' => Tile::S,
			_ => unreachable!(),
		}
	}
}

impl Tile {
	fn moves(&self) -> Vec<(i32, i32)> {
		match self {
			Tile::NS => vec![(0, -1), (0, 1)],
			Tile::WE => vec![(-1, 0), (1, 0)],
			Tile::NE => vec![(0, -1), (1, 0)],
			Tile::NW => vec![(0, -1), (-1, 0)],
			Tile::SW => vec![(0, 1), (-1, 0)],
			Tile::SE => vec![(0, 1), (1, 0)],
			Tile::G => vec![],
			Tile::S => vec![(-1, 0), (0, -1), (0, 1), (1, 0)],
		}
	}
}

impl Grid<Tile> {
	fn find_start(&self) -> (usize, usize) {
		let idx = self.cells.iter().position(|tile| matches!(tile, Tile::S)).unwrap();
		(idx % self.width, idx / self.width)
	}

	fn available_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
		let mut moves = Vec::new();
		for (i, j) in self.get(Point(x as i32, y as i32)).moves() {
			let (xx, yy) = (x as i32 + i, y as i32 + j);
			if xx < 0 || yy < 0 || xx >= self.width as i32 || yy >= self.height as i32 {
				continue;
			}
			if !self.get(Point(xx, yy)).moves().contains(&(-i, -j)) {
				continue;
			}
			moves.push((xx as usize, yy as usize))
		}
		moves
	}
}

pub fn parse_input(input: &str) -> Grid<Tile> {
	Grid::new(input)
}

pub fn part1(input: &Grid<Tile>) -> usize {
	let mut frontier = Vec::new();
	let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
	let mut cost: HashMap<(usize, usize), usize> = HashMap::new();
	let start = input.find_start();
	frontier.push(start);
	came_from.insert(start, start);
	cost.insert(start, 0);

	while let Some(current) = frontier.pop() {
		for next in input.available_moves(current.0, current.1) {
			let new_cost = cost.get(&current).unwrap() + 1;
			if !cost.contains_key(&next) || new_cost < *cost.get(&next).unwrap() {
				cost.insert(next, new_cost);
				frontier.push(next);
				came_from.insert(next, current);
			}
		}
	}

	*cost.values().max().unwrap()
}

fn inside(x: i32, y: i32, polygon: &[(i32, i32)]) -> bool {
	let mut inside = false;
	let mut j = polygon.len() - 1;

	for i in 0..polygon.len() {
		if (polygon[i].1 > y) != (polygon[j].1 > y)
			&& x < (polygon[j].0 - polygon[i].0) * (y - polygon[i].1) / (polygon[j].1 - polygon[i].1) + polygon[i].0
		{
			inside = !inside;
		}

		j = i;
	}
	inside
}

pub fn part2(input: &Grid<Tile>) -> usize {
	let start = input.find_start();
	let mut current = start;
	let mut previous = start;
	let mut corners = vec![(start.0 as i32, start.1 as i32)];
	let mut tiles = vec![start];

	loop {
		let next = *input
			.available_moves(current.0, current.1)
			.iter()
			.find(|&&pos| pos != previous)
			.unwrap();

		if next == start {
			break;
		}

		tiles.push(next);
		if matches!(
			input.get(Point(next.0 as i32, next.1 as i32)),
			Tile::NE | Tile::NW | Tile::SE | Tile::SW
		) {
			corners.push((next.0 as i32, next.1 as i32));
		}

		previous = current;
		current = next;
	}

	let mut res = 0;
	for x in 0..input.width {
		for y in 0..input.height {
			if tiles.contains(&(x, y)) {
				continue;
			}

			if inside(x as i32, y as i32, &corners) {
				res += 1;
			}
		}
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

	const EXAMPLE2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

	const EXAMPLE3: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

	const EXAMPLE4: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

	const EXAMPLE5: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

	const EXAMPLE6: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

	const EXAMPLE7: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

	const EXAMPLE8: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 4);
		assert_eq!(part1(&parse_input(EXAMPLE2)), 8);
		assert_eq!(part1(&parse_input(EXAMPLE3)), 4);
		assert_eq!(part1(&parse_input(EXAMPLE4)), 8);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE5)), 4);
		assert_eq!(part2(&parse_input(EXAMPLE6)), 4);
		assert_eq!(part2(&parse_input(EXAMPLE7)), 8);
		assert_eq!(part2(&parse_input(EXAMPLE8)), 10);
	}
}
