use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
	input
		.lines()
		.map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
		.collect()
}

pub fn part1(input: &[Vec<u64>]) -> u64 {
	let mut fst = Vec::new();
	let mut snd = Vec::new();

	for v in input {
		fst.push(v[0]);
		snd.push(v[1]);
	}

	fst.sort();
	snd.sort();

	let mut res = 0;
	for (a, b) in fst.iter().zip(&snd) {
		res += a.max(b) - a.min(b);
	}
	res
}

pub fn part2(input: &[Vec<u64>]) -> u64 {
	let mut snd = HashMap::new();

	for v in input {
		*snd.entry(v[1]).or_insert(0_u64) += 1;
	}

	let mut res = 0;
	for v in input {
		let a = v[0];
		let b = *snd.get(&v[0]).unwrap_or(&0);
		// dbg!(a, b, a * b);
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
