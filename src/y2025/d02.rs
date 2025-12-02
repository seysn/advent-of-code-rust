use std::ops::RangeInclusive;

pub fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
	input
		.split(',')
		.map(|r| r.split_once('-').unwrap())
		.map(|(a, b)| RangeInclusive::new(a.parse().unwrap(), b.parse().unwrap()))
		.collect()
}

pub fn part1(input: &[RangeInclusive<u64>]) -> u64 {
	let mut res = 0;

	for r in input {
		let mut start = *r.start();
		let mut end = *r.end();

		let start_n = start.ilog10() + 1;
		let end_n = end.ilog10() + 1;

		if !start_n.is_multiple_of(2) {
			if start_n == end_n {
				continue;
			}

			start = 10u64.pow(start_n);
		}

		if !end_n.is_multiple_of(2) {
			end = 10u64.pow(end_n - 1) - 1;
		}

		if start > end {
			continue;
		}

		for i in start..=end {
			let n = i.ilog10() + 1;
			if !n.is_multiple_of(2) {
				continue;
			}

			let right = i % 10u64.pow(n / 2);
			let left = i / 10u64.pow(n / 2);

			if right == left {
				res += i;
			}
		}
	}

	res
}

fn is_repeating(id: u64) -> bool {
	let n = id.ilog10() + 1;
	let id = id.to_string().chars().collect::<Vec<_>>();
	for i in 1..=n / 2 {
		if !n.is_multiple_of(i) {
			continue;
		}

		if id
			.chunks_exact(i as usize)
			.zip(id.chunks_exact(i as usize).skip(1))
			.all(|(a, b)| a == b)
		{
			return true;
		}
	}

	false
}

pub fn part2(input: &[RangeInclusive<u64>]) -> u64 {
	let mut res = 0;

	for r in input {
		for i in r.clone() {
			if is_repeating(i) {
				res += i;
			}
		}
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 1227775554);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 4174379265);
	}
}
