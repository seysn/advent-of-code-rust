use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<u64> {
	input.split(' ').map(|n| n.parse().unwrap()).collect()
}

enum Step {
	Splited(u64, u64),
	Changed(u64),
}

fn step(stone: u64) -> Step {
	if stone == 0 {
		return Step::Changed(stone + 1);
	}

	let numbers = stone.ilog10() as u64 + 1;
	if numbers.is_multiple_of(2) {
		let p = 10u64.pow(numbers as u32 / 2);
		Step::Splited(stone / p, stone % p)
	} else {
		Step::Changed(stone * 2024)
	}
}

fn count(stone: u64, blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
	if let Some(res) = cache.get(&(stone, blinks)) {
		return *res;
	}

	if blinks == 0 {
		return 1;
	}

	match step(stone) {
		Step::Splited(a, b) => {
			let aa = count(a, blinks - 1, cache);
			cache.insert((a, blinks - 1), aa);
			let bb = count(b, blinks - 1, cache);
			cache.insert((b, blinks - 1), bb);

			aa + bb
		}
		Step::Changed(a) => {
			let aa = count(a, blinks - 1, cache);
			cache.insert((a, blinks - 1), aa);

			aa
		}
	}
}

fn solve(input: &[u64], blinks: u64) -> u64 {
	input.iter().map(|i| count(*i, blinks, &mut HashMap::new())).sum()
}

pub fn part1(input: &[u64]) -> u64 {
	solve(input, 25)
}

pub fn part2(input: &[u64]) -> u64 {
	solve(input, 75)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "125 17";

	#[test]
	fn example_part1() {
		assert_eq!(solve(&parse_input(EXAMPLE), 6), 22);
		assert_eq!(solve(&parse_input(EXAMPLE), 25), 55312);
	}
}
