use std::collections::HashMap;

use regex::Regex;

pub struct Documents {
	instructions: String,
	maps: HashMap<String, (String, String)>,
}

pub fn parse_input(input: &str) -> Documents {
	let re = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();

	let mut lines = input.lines();
	let instructions = lines.next().unwrap().to_string();

	let mut maps = HashMap::new();
	lines.skip(1).for_each(|l| {
		let cap = re.captures(l).unwrap();
		maps.entry(cap[1].to_string()).or_insert((cap[2].to_string(), cap[3].to_string()));
	});

	Documents { instructions, maps }
}

pub fn part1(input: &Documents) -> usize {
	let mut node = input.maps.get_key_value("AAA").unwrap().0;

	for (i, instruction) in input.instructions.chars().cycle().enumerate() {
		node = match instruction {
			'L' => &input.maps.get(node).unwrap().0,
			'R' => &input.maps.get(node).unwrap().1,
			_ => unreachable!(),
		};

		if node == "ZZZ" {
			return i + 1;
		}
	}

	unreachable!()
}

#[allow(unused)]
fn walk(input: &Documents, start: &str) -> (usize, String) {
	let mut node = start;

	for (i, instruction) in input.instructions.chars().cycle().enumerate() {
		let tmp = match instruction {
			'L' => &input.maps.get(node).unwrap().0,
			'R' => &input.maps.get(node).unwrap().1,
			_ => unreachable!(),
		};

		if node == tmp {
			return (usize::MAX, node.to_string());
		}

		node = tmp;

		if node.ends_with('Z') {
			return (i + 1, node.to_string());
		}
	}

	unreachable!()
}

#[allow(unused)]
pub fn part2(input: &Documents) -> usize {
	// TODO: Optimize

	// let mut map: HashMap<&String, (usize, String)> = HashMap::new();
	// for node in input.maps.keys() {
	// 	map.insert(node, walk(input, node));
	// }

	// let mut nodes: Vec<(usize, String)> = input
	// 	.maps
	// 	.keys()
	// 	.filter(|k| k.ends_with('A'))
	// 	.map(|k| (map.get(k).unwrap().0, map.get(k).unwrap().1.to_string()))
	// 	.collect();
	// let mut max = nodes.iter().map(|(a, _)| *a).max().unwrap();

	// let mut i = 0;
	// while i < nodes.len() {
	// 	while nodes[i].0 < max {
	// 		let res = map.get(&nodes[i].1).unwrap();
	// 		nodes[i] = (nodes[i].0 + res.0, res.1.to_string());
	// 	}

	// 	if nodes[i].0 > max {
	// 		max = nodes[i].0;
	// 		i = 0
	// 	} else {
	// 		i += 1;
	// 	}
	// }

	// max

	6
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

	const EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

	const EXAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 2);
		assert_eq!(part1(&parse_input(EXAMPLE2)), 6);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE3)), 6);
	}
}
