use std::collections::HashMap;

#[derive(Debug)]
pub struct Towels {
	patterns: Vec<String>,
	designs: Vec<String>,
}

pub fn parse_input(input: &str) -> Towels {
	let (patterns, desired) = input.split_once("\n\n").unwrap();

	Towels {
		patterns: patterns.split(", ").map(|s| s.to_owned()).collect(),
		designs: desired.lines().map(|s| s.to_owned()).collect(),
	}
}

// We could use `ways` for part2 but this one is ~5 times faster for part1 because of early return
fn is_possible(design: &str, patterns: &[String], cache: &mut HashMap<String, bool>) -> bool {
	if let Some(res) = cache.get(design) {
		return *res;
	}

	for pattern in patterns {
		if let Some(s) = design.strip_prefix(pattern) {
			if s.is_empty() || is_possible(s, patterns, cache) {
				cache.insert(s.to_owned(), true);
				return true;
			}
		}
	}

	cache.insert(design.to_owned(), false);
	false
}

pub fn part1(input: &Towels) -> usize {
	let mut cache = HashMap::new();

	input
		.designs
		.iter()
		.filter(|design| is_possible(design, &input.patterns, &mut cache))
		.count()
}

fn ways(design: &str, patterns: &[String], cache: &mut HashMap<String, u64>) -> u64 {
	if let Some(res) = cache.get(design) {
		return *res;
	}

	let mut res = 0;
	for pattern in patterns {
		if let Some(s) = design.strip_prefix(pattern) {
			if s.is_empty() {
				res += 1;
			} else {
				res += ways(s, patterns, cache);
			}
		}
	}

	cache.insert(design.to_owned(), res);
	res
}

pub fn part2(input: &Towels) -> u64 {
	let mut cache = HashMap::new();

	input.designs.iter().map(|design| ways(design, &input.patterns, &mut cache)).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 6);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 16);
	}
}
