use itertools::Itertools;

pub struct Position {
	x: usize,
	y: usize,
}

pub fn parse_input(input: &str) -> Vec<Position> {
	let mut res = Vec::new();
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c == '#' {
				res.push(Position { x, y });
			}
		}
	}
	res
}

fn expand_positions(positions: &[Position], expansion: usize) -> Vec<Position> {
	let xs: Vec<usize> = positions.iter().map(|pos| pos.x).collect();
	let ys: Vec<usize> = positions.iter().map(|pos| pos.y).collect();
	let empty_x: Vec<usize> = (0..*xs.iter().max().unwrap()).filter(|x| !xs.contains(x)).collect();
	let empty_y: Vec<usize> = (0..*ys.iter().max().unwrap()).filter(|y| !ys.contains(y)).collect();
	let multiplier = if expansion == 1 { 1 } else { expansion - 1 };

	positions
		.iter()
		.map(|pos| Position {
			x: pos.x + multiplier * empty_x.iter().filter(|&&x| x < pos.x).count(),
			y: pos.y + multiplier * empty_y.iter().filter(|&&y| y < pos.y).count(),
		})
		.collect()
}

fn sum_paths(positions: &[Position]) -> usize {
	positions
		.iter()
		.combinations(2)
		.map(|pos| pos[0].x.max(pos[1].x) - pos[0].x.min(pos[1].x) + pos[0].y.max(pos[1].y) - pos[0].y.min(pos[1].y))
		.sum()
}

pub fn part1(input: &[Position]) -> usize {
	sum_paths(&expand_positions(input, 1))
}

pub fn part2(input: &[Position]) -> usize {
	sum_paths(&expand_positions(input, 1_000_000))
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 374);
	}

	#[test]
	fn example_part2() {
		let example = parse_input(EXAMPLE);
		assert_eq!(sum_paths(&expand_positions(&example, 10)), 1030);
		assert_eq!(sum_paths(&expand_positions(&example, 100)), 8410);
	}
}
