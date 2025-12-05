use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Database {
	id_ranges: Vec<RangeInclusive<u64>>,
	available_ids: Vec<u64>,
}

pub fn parse_input(input: &str) -> Database {
	let (ranges, availables) = input.split_once("\n\n").unwrap();

	let id_ranges = ranges
		.lines()
		.map(|l| {
			let (start, end) = l.split_once('-').unwrap();
			start.parse().unwrap()..=end.parse().unwrap()
		})
		.collect();

	let available_ids = availables.lines().map(|l| l.parse().unwrap()).collect();

	Database { id_ranges, available_ids }
}

fn is_fresh(id: &u64, ranges: &[RangeInclusive<u64>]) -> bool {
	for range in ranges {
		if range.contains(id) {
			return true;
		}
	}

	false
}

pub fn part1(input: &Database) -> usize {
	input.available_ids.iter().filter(|id| is_fresh(id, &input.id_ranges)).count()
}

pub fn part2(input: &Database) -> u64 {
	let mut ranges = input.id_ranges.clone();
	ranges.sort_by(|a, b| a.start().cmp(b.start()));

	let mut merged = Vec::new();
	let mut current = ranges[0].clone();

	for range in &ranges[1..] {
		if range.start() <= current.end() {
			current = *current.start()..=*current.end().max(range.end());
		} else {
			merged.push(current);
			current = range.clone();
		}
	}
	merged.push(current);

	let mut res = 0;
	for range in merged {
		res += range.end() - range.start() + 1;
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 3);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 14);
	}
}
