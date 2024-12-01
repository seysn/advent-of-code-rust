use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
	input
		.lines()
		.map(|l| l.split_once("   ").unwrap())
		.map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
		.collect()
}

pub fn part1(input: &[(u64, u64)]) -> u64 {
	let mut fst = Vec::new();
	let mut snd = Vec::new();

	for (a, b) in input {
		fst.push(*a);
		snd.push(*b);
	}

	fst.sort();
	snd.sort();

	let mut res = 0;
	for (a, b) in fst.iter().zip(&snd) {
		res += a.max(b) - a.min(b);
	}
	res
}

pub fn part2(input: &[(u64, u64)]) -> u64 {
	let mut snd = HashMap::new();

	for (_, b) in input {
		*snd.entry(b).or_insert(0_u64) += 1;
	}

	let mut res = 0;
	for (a, _) in input {
		let b = *snd.get(&a).unwrap_or(&0);
		res += a * b;
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 11);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 31);
	}
}
