use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<String> {
	input.lines().map(|l| l.to_owned()).collect()
}

pub fn part1(input: &[String]) -> String {
	let mut res = String::new();
	for i in 0..input[0].len() {
		let mut counter: HashMap<char, usize> = HashMap::new();
		for line in input {
			*counter.entry(line.chars().nth(i).unwrap()).or_default() += 1;
		}

		let (ch, _) = counter.iter().max_by_key(|(_, i)| **i).unwrap();
		res.push(*ch);
	}

	res
}

pub fn part2(input: &[String]) -> String {
	let mut res = String::new();
	for i in 0..input[0].len() {
		let mut counter: HashMap<char, usize> = HashMap::new();
		for line in input {
			*counter.entry(line.chars().nth(i).unwrap()).or_default() += 1;
		}

		let (ch, _) = counter.iter().min_by_key(|(_, i)| **i).unwrap();
		res.push(*ch);
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), "easter".to_owned());
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), "advent".to_owned());
	}
}
