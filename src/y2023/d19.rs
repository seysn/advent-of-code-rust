use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Clone)]
enum Action {
	Move(String),
	Accepted,
	Rejected,
}

#[derive(Clone)]
enum Rule {
	LessThan(String, u64, Action),
	GreaterThan(String, u64, Action),
	Always(Action),
}

struct Workflow {
	name: String,
	rules: Vec<Rule>,
}

pub struct System {
	workflows: HashMap<String, Workflow>,
	parts_list: Vec<HashMap<String, u64>>,
}

impl From<&str> for Action {
	fn from(value: &str) -> Self {
		match value {
			"A" => Action::Accepted,
			"R" => Action::Rejected,
			v => Action::Move(v.to_string()),
		}
	}
}

impl From<&str> for Rule {
	fn from(value: &str) -> Self {
		if let Some(pos) = value.find(':') {
			if let Some(pos2) = value.find('>') {
				Self::GreaterThan(
					value[..pos2].to_string(),
					value[pos2 + 1..pos].parse().unwrap(),
					value[pos + 1..].into(),
				)
			} else if let Some(pos2) = value.find('<') {
				Self::LessThan(
					value[..pos2].to_string(),
					value[pos2 + 1..pos].parse().unwrap(),
					value[pos + 1..].into(),
				)
			} else {
				unreachable!()
			}
		} else {
			Self::Always(value.into())
		}
	}
}

impl From<&str> for Workflow {
	fn from(value: &str) -> Self {
		let start = value.find('{').unwrap();
		let end = value.len() - 1;
		Workflow {
			name: value[0..start].to_string(),
			rules: value[start + 1..end].split(',').map(|r| r.into()).collect(),
		}
	}
}

impl Workflow {
	fn process(&self, parts: &HashMap<String, u64>) -> Action {
		for rule in &self.rules {
			match rule {
				Rule::LessThan(s, v, a) => {
					if parts.get(s).unwrap() < v {
						return a.clone();
					}
				}
				Rule::GreaterThan(s, v, a) => {
					if parts.get(s).unwrap() > v {
						return a.clone();
					}
				}
				Rule::Always(a) => {
					return a.clone();
				}
			}
		}

		unreachable!()
	}
}

pub fn parse_input(input: &str) -> System {
	let mut splited = input.split("\n\n");

	let mut workflows = HashMap::new();
	for workflow in splited.next().unwrap().lines().map(Workflow::from) {
		workflows.insert(workflow.name.clone(), workflow);
	}

	let parts_list = splited
		.next()
		.unwrap()
		.lines()
		.map(|l| {
			let mut parts = HashMap::new();
			for s in l[1..l.len() - 1].split(',') {
				if let Some(idx) = s.find('=') {
					let part = s[..idx].to_string();
					let value = s[idx + 1..].parse().unwrap();
					parts.insert(part, value);
				}
			}
			parts
		})
		.collect();

	System { workflows, parts_list }
}

pub fn part1(input: &System) -> u64 {
	let mut res = 0;
	for parts in &input.parts_list {
		let mut workflow = input.workflows.get("in").unwrap();
		loop {
			match workflow.process(parts) {
				Action::Move(w) => {
					workflow = input.workflows.get(&w).unwrap();
				}
				Action::Accepted => {
					res += parts.values().sum::<u64>();
					break;
				}
				Action::Rejected => break,
			}
		}
	}

	res
}

fn product_parts(parts: &HashMap<String, RangeInclusive<u64>>) -> u64 {
	let mut res = 1;
	for v in parts.values() {
		res *= v.end() - v.start() + 1;
	}
	res
}

impl System {
	fn range_accepted(&self, parts: HashMap<String, RangeInclusive<u64>>, start: &str) -> u64 {
		let mut res = Vec::new();
		let workflow = self.workflows.get(start).unwrap();
		let mut current = parts.clone();

		for rule in &workflow.rules {
			match rule {
				Rule::LessThan(s, v, a) => {
					let part_range = current.get(s).unwrap();
					let mut new = current.clone();
					new.insert(s.to_string(), *part_range.start()..=v - 1);
					current.insert(s.to_string(), *v..=*part_range.end());

					match a {
						Action::Move(w) => {
							res.push(self.range_accepted(new, w));
						}
						Action::Accepted => {
							res.push(product_parts(&new));
						}
						Action::Rejected => {}
					}
				}
				Rule::GreaterThan(s, v, a) => {
					let part_range = current.get(s).unwrap();
					let mut new = current.clone();
					new.insert(s.to_string(), v + 1..=*part_range.end());
					current.insert(s.to_string(), *part_range.start()..=*v);

					match a {
						Action::Move(w) => {
							res.push(self.range_accepted(new, w));
						}
						Action::Accepted => {
							res.push(product_parts(&new));
						}
						Action::Rejected => {}
					}
				}
				Rule::Always(a) => match a {
					Action::Move(w) => {
						res.push(self.range_accepted(current.clone(), w));
					}
					Action::Accepted => {
						res.push(product_parts(&current));
					}
					Action::Rejected => {}
				},
			}
		}

		res.iter().sum()
	}
}

pub fn part2(input: &System) -> u64 {
	let mut parts = HashMap::new();
	parts.insert("x".to_string(), 1..=4000);
	parts.insert("m".to_string(), 1..=4000);
	parts.insert("a".to_string(), 1..=4000);
	parts.insert("s".to_string(), 1..=4000);

	input.range_accepted(parts, "in")
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 19114);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 167409079868000);
	}
}
