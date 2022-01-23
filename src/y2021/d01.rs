pub fn parse_input(content: &str) -> Vec<i32> {
	content.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: &[i32]) -> i32 {
	input
		.iter()
		.zip(input.iter().skip(1))
		.fold(0, |acc, (x, y)| if x < y { acc + 1 } else { acc })
}

pub fn part2(input: &[i32]) -> i32 {
	let mut res = 0;
	let mut prev: i32 = input.iter().take(3).sum();
	for i in 1..input.len() {
		let current = input.iter().skip(i).take(3).sum();
		if prev < current {
			res += 1;
		}
		prev = current;
	}
	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &'static str = "199
200
208
210
200
207
240
269
260
263";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 7);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 5);
	}
}
