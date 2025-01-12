use regex::Regex;

pub fn parse_input(input: &str) -> (u32, u32) {
	let re = Regex::new(r"To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).").unwrap();
	let caps = re.captures(input).unwrap();
	let a = caps[1].parse().unwrap();
	let b = caps[2].parse().unwrap();

	(a, b)
}

pub fn part1(input: &(u32, u32)) -> u64 {
	let mut last = 20151125;
	for row_start in 2..u32::MAX {
		for (i, col) in (1..=row_start).enumerate() {
			let row = row_start - i as u32;
			last = (last * 252533) % 33554393;
			if row == input.0 && col == input.1 {
				return last;
			}
		}
	}

	0
}

pub fn part2(_: &(u32, u32)) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&(2, 2)), 21629792);
		assert_eq!(part1(&(6, 6)), 27995004);
	}
}
