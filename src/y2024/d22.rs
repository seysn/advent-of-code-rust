use std::collections::{HashMap, HashSet, VecDeque};

pub fn parse_input(input: &str) -> Vec<u64> {
	input.lines().filter_map(|l| l.parse().ok()).collect()
}

fn next_secret(mut number: u64) -> u64 {
	number = ((number * 64) ^ number) % 16777216;
	number = ((number / 32) ^ number) % 16777216;
	number = ((number * 2048) ^ number) % 16777216;

	number
}

fn generate(mut number: u64, n: usize) -> u64 {
	for _ in 0..n {
		number = next_secret(number)
	}
	number
}

pub fn part1(input: &[u64]) -> u64 {
	input.iter().map(|n| generate(*n, 2000)).sum()
}

pub fn part2(input: &[u64]) -> u64 {
	let mut bananas: HashMap<(i32, i32, i32, i32), u64> = HashMap::new();
	for number in input {
		let mut last = *number;

		let mut done: HashSet<(i32, i32, i32, i32)> = HashSet::new();

		let mut deltas: VecDeque<i32> = VecDeque::new();
		for _ in 1..2000 {
			let next = next_secret(last);
			let value = next % 10;

			let delta = value as i32 - (last % 10) as i32;
			if deltas.len() == 4 {
				deltas.pop_front();
			}
			deltas.push_back(delta);
			if deltas.len() == 4 {
				let key = (deltas[0], deltas[1], deltas[2], deltas[3]);
				if done.insert(key) {
					*bananas.entry(key).or_default() += value;
				}
			}

			last = next;
		}
	}

	*bananas.values().max().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(next_secret(123), 15887950);
		assert_eq!(next_secret(15887950), 16495136);
		assert_eq!(next_secret(16495136), 527345);

		assert_eq!(part1(&[1, 10, 100, 2024]), 37327623);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&[1, 2, 3, 2024]), 23);
	}
}
