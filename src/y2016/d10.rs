use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Target {
	Bot(u32),
	Output(u32),
}

#[derive(Debug)]
pub enum Instruction {
	Value(u32, u32),
	Give(u32, Target, Target),
}

impl Target {
	fn new(target: &str, value: u32) -> Self {
		match target {
			"bot" => Self::Bot(value),
			"output" => Self::Output(value),
			_ => unreachable!("{target}"),
		}
	}
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|l| {
			let words: Vec<&str> = l.split(' ').collect();

			match words[0] {
				"value" => Instruction::Value(words[1].parse().unwrap(), words[5].parse().unwrap()),
				"bot" => {
					let low = Target::new(words[5], words[6].parse().unwrap());
					let high = Target::new(words[10], words[11].parse().unwrap());

					Instruction::Give(words[1].parse().unwrap(), low, high)
				}
				_ => unreachable!("{words:?}"),
			}
		})
		.collect()
}

struct Bot {
	id: u32,
	values: Vec<u32>,
	low: Option<Target>,
	high: Option<Target>,
}

impl Bot {
	fn new(id: u32) -> Self {
		Self {
			id,
			values: Vec::new(),
			low: None,
			high: None,
		}
	}
}

fn solve(input: &[Instruction], watch: (u32, u32)) -> u32 {
	let mut bots = HashMap::new();
	for inst in input {
		match inst {
			Instruction::Value(v, id) => {
				bots.entry(*id).or_insert(Bot::new(*id)).values.push(*v);
			}
			Instruction::Give(id, low, high) => {
				let bot = bots.entry(*id).or_insert(Bot::new(*id));
				bot.low = Some(low.clone());
				bot.high = Some(high.clone());
			}
		}
	}

	let ids: Vec<u32> = bots.keys().cloned().collect();
	for id in ids.iter().cycle() {
		let bot = bots.get_mut(id).unwrap();
		if bot.values.len() != 2 {
			continue;
		}

		let a = bot.values[0];
		let b = bot.values[1];
		bot.values.clear();

		let (min, max) = if a < b { (a, b) } else { (b, a) };

		if watch.0.min(watch.1) == min && watch.0.max(watch.1) == max {
			return bot.id;
		}

		let low = bot.low.clone();
		let high = bot.high.clone();

		if let Some(Target::Bot(next)) = low {
			bots.get_mut(&next).unwrap().values.push(min);
		}

		if let Some(Target::Bot(next)) = high {
			bots.get_mut(&next).unwrap().values.push(max);
		}
	}

	unreachable!()
}

pub fn part1(input: &[Instruction]) -> u32 {
	solve(input, (17, 61))
}

fn solve2(input: &[Instruction]) -> HashMap<u32, u32> {
	let mut bots = HashMap::new();
	for inst in input {
		match inst {
			Instruction::Value(v, id) => {
				bots.entry(*id).or_insert(Bot::new(*id)).values.push(*v);
			}
			Instruction::Give(id, low, high) => {
				let bot = bots.entry(*id).or_insert(Bot::new(*id));
				bot.low = Some(low.clone());
				bot.high = Some(high.clone());
			}
		}
	}

	let mut outputs: HashMap<u32, u32> = HashMap::new();
	let ids: Vec<u32> = bots.keys().cloned().collect();
	loop {
		let mut found = false;
		for id in ids.iter() {
			let bot = bots.get_mut(id).unwrap();
			if bot.values.len() != 2 {
				continue;
			}

			let a = bot.values[0];
			let b = bot.values[1];
			bot.values.clear();

			let (min, max) = if a < b { (a, b) } else { (b, a) };

			let low = bot.low.clone();
			let high = bot.high.clone();

			match low {
				Some(Target::Bot(next)) => {
					bots.get_mut(&next).unwrap().values.push(min);
				}
				Some(Target::Output(idx)) => {
					*outputs.entry(idx).or_default() = min;
				}
				_ => (),
			}

			match high {
				Some(Target::Bot(next)) => {
					bots.get_mut(&next).unwrap().values.push(max);
				}
				Some(Target::Output(idx)) => {
					*outputs.entry(idx).or_default() = max;
				}
				_ => (),
			}

			found = true;
		}

		if !found {
			break;
		}
	}

	outputs
}

pub fn part2(input: &[Instruction]) -> u32 {
	let out = solve2(input);
	out.get(&0).unwrap() * out.get(&1).unwrap() * out.get(&2).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

	#[test]
	fn example_part1() {
		assert_eq!(solve(&parse_input(EXAMPLE), (2, 5)), 2);
	}
}
