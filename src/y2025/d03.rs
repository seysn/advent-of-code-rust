pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
	input
		.lines()
		.map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
		.collect()
}

/// Return the index of the FIRST max value because `Iterator::max` return the LAST
/// if several elements are equally maximum.
fn max_idx(line: &[u64]) -> usize {
	let mut max_idx = 0;
	let mut max_value = 0;

	for (idx, &value) in line.iter().enumerate() {
		if value == 9 {
			// We're not going to find anything larger
			return idx;
		}

		if value > max_value {
			max_idx = idx;
			max_value = value;
		}
	}

	max_idx
}

pub fn part1(input: &[Vec<u64>]) -> u64 {
	let mut res = 0;

	for l in input {
		let fst = max_idx(&l[..l.len() - 1]);
		let snd = max_idx(&l[fst + 1..]) + fst + 1;

		let value = l[fst] * 10 + l[snd];
		res += value;
	}

	res
}

pub fn part2(input: &[Vec<u64>]) -> u64 {
	let mut res = 0;

	for l in input {
		let one = max_idx(&l[..l.len() - 11]);
		let two = max_idx(&l[one + 1..l.len() - 10]) + one + 1;
		let three = max_idx(&l[two + 1..l.len() - 9]) + two + 1;
		let four = max_idx(&l[three + 1..l.len() - 8]) + three + 1;
		let five = max_idx(&l[four + 1..l.len() - 7]) + four + 1;
		let six = max_idx(&l[five + 1..l.len() - 6]) + five + 1;
		let seven = max_idx(&l[six + 1..l.len() - 5]) + six + 1;
		let eight = max_idx(&l[seven + 1..l.len() - 4]) + seven + 1;
		let nine = max_idx(&l[eight + 1..l.len() - 3]) + eight + 1;
		let ten = max_idx(&l[nine + 1..l.len() - 2]) + nine + 1;
		let eleven = max_idx(&l[ten + 1..l.len() - 1]) + ten + 1;
		let twelve = max_idx(&l[eleven + 1..]) + eleven + 1;

		let value = l[one] * 100_000_000_000
			+ l[two] * 10_000_000_000
			+ l[three] * 1_000_000_000
			+ l[four] * 100_000_000
			+ l[five] * 10_000_000
			+ l[six] * 1_000_000
			+ l[seven] * 100_000
			+ l[eight] * 10_000
			+ l[nine] * 1_000
			+ l[ten] * 100
			+ l[eleven] * 10
			+ l[twelve];

		res += value;
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 357);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 3121910778619);
	}
}
