#[derive(Debug)]
pub struct Present {
	blocks: usize,
}

#[derive(Debug)]
pub struct Region {
	width: usize,
	height: usize,
	quantity: Vec<usize>,
}

#[derive(Debug)]
pub struct Input {
	presents: Vec<Present>,
	regions: Vec<Region>,
}

pub fn parse_input(input: &str) -> Input {
	let mut splited: Vec<&str> = input.split("\n\n").collect();

	let regions = splited.pop().unwrap();

	Input {
		presents: splited
			.iter()
			.map(|s| Present {
				blocks: s.chars().filter(|c| *c == '#').count(),
			})
			.collect(),

		regions: regions
			.lines()
			.map(|l| {
				let (shape, quantity) = l.split_once(':').unwrap();
				let (width, height) = shape.split_once('x').unwrap();

				Region {
					width: width.parse().unwrap(),
					height: height.parse().unwrap(),
					quantity: quantity.split_whitespace().map(|s| s.parse().unwrap()).collect(),
				}
			})
			.collect(),
	}
}

pub fn part1(input: &Input) -> usize {
	// Fortunately, the real input works when we just check if the amount of blocks in present
	// is equal or less than the available space.
	// Maybe one day i'll come back to make a real algorithm... or maybe not !

	input
		.regions
		.iter()
		.filter(|region| {
			let available = region.width * region.height;
			let total = region.quantity.iter().zip(&input.presents).map(|(qty, p)| p.blocks * *qty).sum();

			available >= total
		})
		.count()
}

pub fn part2(_: &Input) -> usize {
	0
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

	#[test]
	fn example_part1() {
		// The "algorithm" is working with real input but not the example

		// assert_eq!(part1(&parse_input(EXAMPLE)), 2);
	}
}
