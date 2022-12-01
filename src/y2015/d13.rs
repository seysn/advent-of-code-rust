use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Family {
	attendees: Vec<String>,
	rules: HashMap<(String, String), i32>,
}

pub fn parse_input(input: &str) -> Family {
	let re = Regex::new(r"(\w+) would (lose|gain) (\d+) happiness units by sitting next to (\w+).").unwrap();
	let mut attendees = Vec::new();
	let mut rules = HashMap::new();

	for line in input.lines() {
		let caps = re.captures(line).unwrap();

		let attendee = caps[1].to_string();
		let value: i32 = if &caps[2] == "gain" {
			caps[3].parse().unwrap()
		} else {
			-caps[3].parse::<i32>().unwrap()
		};

		if !attendees.contains(&attendee) {
			attendees.push(attendee.clone());
		}

		rules.insert((attendee, caps[4].to_string()), value);
	}

	Family { attendees, rules }
}

fn happiness(order: &[&String], rules: &HashMap<(String, String), i32>) -> i32 {
	let mut res = 0;
	for (idx, attendee) in order.iter().enumerate() {
		// Left
		res += if idx == 0 {
			rules
				.get(&(attendee.to_string(), order.iter().last().unwrap().to_string()))
				.unwrap()
		} else {
			rules.get(&(attendee.to_string(), order.get(idx - 1).unwrap().to_string())).unwrap()
		};

		// Right
		res += if idx == order.len() - 1 {
			rules.get(&(attendee.to_string(), order.get(0).unwrap().to_string())).unwrap()
		} else {
			rules.get(&(attendee.to_string(), order.get(idx + 1).unwrap().to_string())).unwrap()
		};
	}
	res
}

pub fn part1(input: &Family) -> i32 {
	let mut max = i32::MIN;
	for perm in input.attendees.iter().permutations(input.attendees.len()) {
		let tmp = happiness(&perm, &input.rules);
		if max < tmp {
			max = tmp;
		}
	}
	max
}

pub fn part2(input: &Family) -> i32 {
	let mut new_input = input.clone();
	for attendee in &new_input.attendees {
		new_input.rules.insert(("Pipo".to_string(), attendee.to_string()), 0);
		new_input.rules.insert((attendee.to_string(), "Pipo".to_string()), 0);
	}
	new_input.attendees.push("Pipo".to_string());

	let mut max = i32::MIN;
	for perm in new_input.attendees.iter().permutations(new_input.attendees.len()) {
		let tmp = happiness(&perm, &new_input.rules);
		if max < tmp {
			max = tmp;
		}
	}
	max
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

	#[test]
	fn example() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 330);
	}
}
