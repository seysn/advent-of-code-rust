#[derive(Clone)]
pub struct Grid {
	trees: Vec<u32>,
	width: usize,
	height: usize,
}

impl Grid {
	fn get(&self, x: usize, y: usize) -> u32 {
		self.trees[self.width * y + x]
	}

	fn set(&mut self, x: usize, y: usize, value: u32) {
		self.trees[self.width * y + x] = value;
	}

	fn inc(&mut self, x: usize, y: usize) {
		self.trees[self.width * y + x] += 1;
	}

	fn scenic_score(&self, x: usize, y: usize) -> u32 {
		let value = self.get(x, y);
		let mut res = 1;
		let mut cpt = 0;

		// Up
		for j in (0..y).rev() {
			cpt += 1;
			if self.get(x, j) >= value {
				break;
			}
		}
		res *= cpt;
		cpt = 0;

		// Down
		for j in y + 1..self.height {
			cpt += 1;
			if self.get(x, j) >= value {
				break;
			}
		}
		res *= cpt;
		cpt = 0;

		// Left
		for i in (0..x).rev() {
			cpt += 1;
			if self.get(i, y) >= value {
				break;
			}
		}
		res *= cpt;
		cpt = 0;

		// Right
		for i in x + 1..self.width {
			cpt += 1;
			if self.get(i, y) >= value {
				break;
			}
		}
		res *= cpt;

		res
	}
}

pub fn parse_input(input: &str) -> Grid {
	let trees: Vec<u32> = input.chars().filter(|c| c.is_numeric()).map(|c| c as u32 - 48).collect();
	let width = input.lines().next().unwrap().len();
	let height = trees.len() / width;

	Grid { trees, width, height }
}

pub fn part1(input: &Grid) -> usize {
	let mut visibles = Grid {
		trees: vec![0; input.width * input.height],
		width: input.width,
		height: input.height,
	};

	// Inc top and bottom borders
	for x in 0..input.width {
		visibles.inc(x, 0);
		visibles.inc(x, input.height - 1);
	}

	// Inc left and right borders
	for y in 0..input.width {
		visibles.inc(0, y);
		visibles.inc(input.width - 1, y);
	}

	let mut max;
	for x in 1..input.width - 1 {
		// From top to bottom
		max = input.get(x, 0);
		for y in 1..input.height {
			let tree = input.get(x, y);
			if tree > max {
				visibles.inc(x, y);
				max = tree;
			}
		}

		// From bottom to top
		max = input.get(x, input.height - 1);
		for y in (0..input.height - 1).rev() {
			let tree = input.get(x, y);
			if tree > max {
				visibles.inc(x, y);
				max = tree;
			}
		}
	}

	for y in 1..input.height - 1 {
		// From left to right
		max = input.get(0, y);
		for x in 1..input.width {
			let tree = input.get(x, y);
			if tree > max {
				visibles.inc(x, y);
				max = tree;
			}
		}

		// From right to left
		max = input.get(input.width - 1, y);
		for x in (0..input.width - 1).rev() {
			let tree = input.get(x, y);
			if tree > max {
				visibles.inc(x, y);
				max = tree;
			}
		}
	}

	visibles.trees.iter().filter(|&&x| x > 0).count()
}

pub fn part2(input: &Grid) -> u32 {
	let mut visibles = Grid {
		trees: vec![1; input.width * input.height],
		width: input.width,
		height: input.height,
	};

	for x in 1..input.width - 1 {
		for y in 1..input.height - 1 {
			visibles.set(x, y, input.scenic_score(x, y));
		}
	}

	*visibles.trees.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "30373
25512
65332
33549
35390";

	#[test]
	fn example_get() {
		let grid = parse_input(EXAMPLE);
		assert_eq!(grid.get(0, 0), 3);
		assert_eq!(grid.get(1, 1), 5);
		assert_eq!(grid.get(3, 0), 7);
		assert_eq!(grid.get(0, 2), 6);
	}

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 21);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 8);
	}
}
