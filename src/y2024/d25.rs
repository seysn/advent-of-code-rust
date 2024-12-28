#[derive(Debug)]
pub struct Lock([u8; 5]);

#[derive(Debug)]
pub struct Key([u8; 5]);

pub fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
	let mut locks = Vec::new();
	let mut keys = Vec::new();

	for block in input.split("\n\n") {
		let mut heights = [0; 5];
		let lines: Vec<&str> = block.lines().collect();

		for line in lines.iter().take(lines.len() - 1).skip(1) {
			for (j, ch) in line.chars().enumerate() {
				if ch == '#' {
					heights[j] += 1;
				}
			}
		}

		if lines[0] == "#####" {
			locks.push(Lock(heights));
		} else {
			keys.push(Key(heights));
		}
	}

	(locks, keys)
}

pub fn part1(input: &(Vec<Lock>, Vec<Key>)) -> usize {
	let mut res = 0;
	for lock in &input.0 {
		'keys: for key in &input.1 {
			for (a, b) in lock.0.iter().zip(&key.0) {
				if a + b > 5 {
					continue 'keys;
				}
			}
			res += 1;
		}
	}

	res
}

pub fn part2(_: &(Vec<Lock>, Vec<Key>)) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 3);
	}
}
