pub fn parse_input(input: &str) -> String {
	input.to_owned()
}

fn decompressed_length(s: &str) -> usize {
	if s.is_empty() {
		return 0;
	}

	if s.starts_with('(') {
		let end = s.find(')').unwrap();
		let (a, b) = s[1..end].split_once('x').unwrap();

		let take: usize = a.parse().unwrap();
		let repeat: usize = b.parse().unwrap();

		take * repeat + decompressed_length(&s[end + take + 1..])
	} else {
		1 + decompressed_length(&s[1..])
	}
}

pub fn part1(input: &str) -> usize {
	decompressed_length(input)
}

fn decompressed_length_v2(s: &str) -> usize {
	if s.is_empty() {
		return 0;
	}

	if s.starts_with('(') {
		let end = s.find(')').unwrap();
		let (a, b) = s[1..end].split_once('x').unwrap();

		let take: usize = a.parse().unwrap();
		let repeat: usize = b.parse().unwrap();

		decompressed_length_v2(&s[end + 1..end + take + 1]) * repeat + decompressed_length_v2(&s[end + take + 1..])
	} else {
		1 + decompressed_length_v2(&s[1..])
	}
}

pub fn part2(input: &str) -> usize {
	decompressed_length_v2(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1("ADVENT"), 6);
		assert_eq!(part1("A(1x5)BC"), 7);
		assert_eq!(part1("(3x3)XYZ"), 9);
		assert_eq!(part1("A(2x2)BCD(2x2)EFG"), 11);
		assert_eq!(part1("(6x1)(1x3)A"), 6);
		assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2("(3x3)XYZ"), 9);
		assert_eq!(part2("X(8x2)(3x3)ABCY"), 20);
		assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
		assert_eq!(part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
	}
}
