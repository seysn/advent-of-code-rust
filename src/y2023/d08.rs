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

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
	if b == 0 {
		return a;
	}
	gcd_of_two_numbers(b, a % b)
}

fn lcm(nums: &[usize]) -> usize {
	if nums.len() == 1 {
		return nums[0];
	}
	let a = nums[0];
	let b = lcm(&nums[1..]);
	a * b / gcd_of_two_numbers(a, b)
}

fn find_cycle(map: &HashMap<&String, (usize, String)>, start: &str) -> usize {
	let mut visited: HashMap<String, usize> = HashMap::new();
	let mut current = start.to_string();
	let mut steps = 0;

	while !visited.contains_key(&current) {
		let (s, k) = map.get(&current).unwrap();
		steps += s;
		current = k.clone();
		visited.insert(k.clone(), steps);
	}

	*visited.get(&current).unwrap()
}

pub fn part2(input: &Documents) -> usize {
	let mut map: HashMap<&String, (usize, String)> = HashMap::new();
	for node in input.maps.keys() {
		map.insert(node, walk(input, node));
	}

	let nodes: Vec<String> = input.maps.keys().filter(|k| k.ends_with('A')).map(|k| k.to_string()).collect();
	let cycles: Vec<usize> = nodes.iter().map(|n| find_cycle(&map, n)).collect();

	lcm(&cycles)
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
