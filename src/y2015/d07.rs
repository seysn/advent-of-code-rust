use std::collections::HashMap;

use regex::Regex;

#[derive(Clone)]
pub enum Instruction {
	Assign(String),
	And(String, String),
	Or(String, String),
	LeftShift(String, u16),
	RightShift(String, u16),
	Not(String),
}

pub fn parse_input(input: &str) -> HashMap<String, Instruction> {
	let mut wires: HashMap<String, Instruction> = HashMap::new();

	let re_assign = Regex::new(r"([[:alnum:]]+) -> (\w+)").unwrap();
	let re_not = Regex::new(r"NOT (\w+) -> (\w+)").unwrap();
	let re_others = Regex::new(r"(\w+) (AND|OR|LSHIFT|RSHIFT) ([[:alnum:]]+) -> (\w+)").unwrap();

	for l in input.lines() {
		if re_others.is_match(l) {
			let caps = re_others.captures(l).unwrap();
			match &caps[2] {
				"AND" => wires.insert(caps[4].to_string(), Instruction::And(caps[1].to_string(), caps[3].to_string())),
				"OR" => wires.insert(caps[4].to_string(), Instruction::Or(caps[1].to_string(), caps[3].to_string())),
				"LSHIFT" => wires.insert(
					caps[4].to_string(),
					Instruction::LeftShift(caps[1].to_string(), caps[3].parse().unwrap()),
				),
				"RSHIFT" => wires.insert(
					caps[4].to_string(),
					Instruction::RightShift(caps[1].to_string(), caps[3].parse().unwrap()),
				),
				_ => unreachable!(),
			};
		} else if re_not.is_match(l) {
			let caps = re_not.captures(l).unwrap();
			wires.insert(caps[2].to_string(), Instruction::Not(caps[1].to_string()));
		} else {
			let caps = re_assign.captures(l).unwrap();
			wires.insert(caps[2].to_string(), Instruction::Assign(caps[1].to_string()));
		}
	}

	wires
}

fn get_signal(wire: &String, wires: &HashMap<String, Instruction>, cache: &mut HashMap<String, u16>) -> u16 {
	if cache.contains_key(wire) {
		return *cache.get(wire).unwrap();
	}

	let parsed: Result<u16, std::num::ParseIntError> = wire.parse();
	if let Ok(value) = parsed {
		return value;
	}

	let inst = wires.get(wire).unwrap();
	let v = match inst {
		Instruction::Assign(a) => get_signal(a, wires, cache),
		Instruction::And(a, b) => get_signal(a, wires, cache) & get_signal(b, wires, cache),
		Instruction::Or(a, b) => get_signal(a, wires, cache) | get_signal(b, wires, cache),
		Instruction::LeftShift(a, b) => get_signal(a, wires, cache) << b,
		Instruction::RightShift(a, b) => get_signal(a, wires, cache) >> b,
		Instruction::Not(a) => !get_signal(a, wires, cache),
	};
	cache.insert(wire.to_string(), v);

	v
}

pub fn part1(input: &HashMap<String, Instruction>) -> u16 {
	let mut cache: HashMap<String, u16> = HashMap::new();
	get_signal(&String::from("a"), input, &mut cache)
}

pub fn part2(input: &HashMap<String, Instruction>) -> u16 {
	let a_signal = part1(input);
	let mut modified_wires = input.clone();
	modified_wires.insert("b".to_string(), Instruction::Assign(a_signal.to_string()));

	let mut cache: HashMap<String, u16> = HashMap::new();
	get_signal(&String::from("a"), &modified_wires, &mut cache)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> a";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 65079);
	}
}
