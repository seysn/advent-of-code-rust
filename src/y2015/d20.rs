pub fn parse_input(input: &str) -> u32 {
	input.parse().unwrap()
}

pub fn part1(input: &u32) -> usize {
	let n = (input / 10) as usize;
	let mut houses = vec![0; n];
	for i in 1..n {
		for j in (i..n).step_by(i) {
			houses[j] += i as u32 * 10;
		}
	}

	for (i, v) in houses.iter().enumerate() {
		if v >= input {
			return i;
		}
	}

	0
}

pub fn part2(input: &u32) -> usize {
	let n = (input / 10) as usize;
	let mut houses = vec![0; n];
	for i in 1..n {
		for j in (i..n).step_by(i).take(50) {
			houses[j] += i as u32 * 11;
		}
	}

	for (i, v) in houses.iter().enumerate() {
		if v >= input {
			return i;
		}
	}

	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&150), 8);
	}
}
