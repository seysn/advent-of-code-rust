pub fn parse_input(input: &str) -> Vec<u8> {
	input.chars().map(|ch| ch as u8 - 48).collect()
}

pub fn part1(input: &[u8]) -> u32 {
	let bounds = if let (Some(first), Some(last)) = (input.first(), input.last())
		&& first == last
	{
		*last as u32
	} else {
		0
	};

	input
		.windows(2)
		.map(|pair| if pair[0] == pair[1] { pair[0] as u32 } else { 0 })
		.sum::<u32>()
		+ bounds
}

pub fn part2(input: &[u8]) -> u32 {
	input[..input.len() / 2]
		.iter()
		.zip(&input[input.len() / 2..])
		.filter_map(|(&a, &b)| if a == b { Some(a as u32) } else { None })
		.sum::<u32>()
		* 2
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input("1122")), 3);
		assert_eq!(part1(&parse_input("1111")), 4);
		assert_eq!(part1(&parse_input("1234")), 0);
		assert_eq!(part1(&parse_input("91212129")), 9);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input("1212")), 6);
		assert_eq!(part2(&parse_input("1221")), 0);
		assert_eq!(part2(&parse_input("123425")), 4);
		assert_eq!(part2(&parse_input("123123")), 12);
		assert_eq!(part2(&parse_input("12131415")), 4);
	}
}
