enum Direction {
	North,
	East,
	South,
	West,
}

#[derive(Clone)]
pub struct Grid {
	lines: Vec<String>,
	rows: Vec<String>,
}

impl PartialEq for Grid {
	fn eq(&self, other: &Self) -> bool {
		self.lines == other.lines
	}
}

impl Grid {
	fn slide(&self, direction: &Direction) -> Grid {
		let rows = match direction {
			Direction::North => self.rows.clone(),
			Direction::East => reverse_vec(&self.lines),
			Direction::South => reverse_vec(&self.rows),
			Direction::West => self.lines.clone(),
		};

		let mut res = Vec::new();
		for row in rows {
			let mut slided_row = String::new();
			let mut spaces = 0;
			for ch in row.chars() {
				match ch {
					'O' => slided_row.push(ch),
					'.' => spaces += 1,
					'#' => {
						for _ in 0..spaces {
							slided_row.push('.');
						}
						slided_row.push('#');
						spaces = 0
					}
					_ => unreachable!(),
				}
			}
			for _ in 0..spaces {
				slided_row.push('.');
			}
			res.push(slided_row);
		}

		match direction {
			Direction::North => Grid {
				lines: tilt_vec(&res),
				rows: res,
			},
			Direction::East => Grid {
				lines: reverse_vec(&res),
				rows: tilt_vec(&reverse_vec(&res)),
			},
			Direction::South => Grid {
				lines: tilt_vec(&reverse_vec(&res)),
				rows: reverse_vec(&res),
			},
			Direction::West => Grid {
				rows: tilt_vec(&res),
				lines: res,
			},
		}
	}

	fn cycle(&self) -> Grid {
		let directions = [Direction::North, Direction::West, Direction::South, Direction::East];
		let mut grid = self.clone();
		for d in directions {
			grid = grid.slide(&d);
		}
		grid
	}

	fn load(&self) -> usize {
		let mut res = 0;
		let height = self.rows[0].len();
		for (i, line) in self.lines.iter().enumerate() {
			res += (height - i) * line.chars().filter(|&c| c == 'O').count();
		}

		res
	}
}

fn tilt_vec(v: &[String]) -> Vec<String> {
	let mut res = vec!["".to_string(); v[0].len()];
	for l in v {
		for (i, c) in l.chars().enumerate() {
			res[i].push(c);
		}
	}

	res
}

fn reverse_vec(v: &[String]) -> Vec<String> {
	v.iter().map(|l| l.chars().rev().collect()).collect()
}

pub fn parse_input(input: &str) -> Grid {
	let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
	let rows: Vec<String> = tilt_vec(&lines);

	Grid { lines, rows }
}

pub fn part1(input: &Grid) -> usize {
	input.slide(&Direction::North).load()
}

pub fn part2(input: &Grid) -> usize {
	let mut grid = input.clone();
	let mut done = vec![grid.clone()];
	for _ in 0..1000000000 {
		grid = grid.cycle();

		if done.contains(&grid) {
			let idx = done.iter().position(|g| g == &grid).unwrap();
			let sub = &done[idx..];
			return sub[(1000000000 - idx) % sub.len()].load();
		}
		done.push(grid.clone());
	}

	grid.load()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 136);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 64);
	}
}
