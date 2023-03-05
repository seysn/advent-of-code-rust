use std::fmt::Debug;

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
	Add(u64),
	Multiply(u64),
	Square,
}

impl Operation {
	fn compute(&self, old: &u64) -> u64 {
		match self {
			Operation::Add(x) => old + x,
			Operation::Multiply(x) => old * x,
			Operation::Square => old * old,
		}
	}
}

#[derive(Clone)]
pub struct Monkey {
	items: Vec<u64>,
	operation: Operation,
	test: u64,
	throw_true: usize,
	throw_false: usize,
	inspected: usize,
}

impl Debug for Monkey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Monkey").field("inspected", &self.inspected).finish()
	}
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
	let mut monkeys = Vec::new();

	for monkey in input.split("\n\n") {
		let mut lines = monkey.lines().skip(1);
		let items = lines.next().unwrap()[18..]
			.split(", ")
			.map(|i| i.parse::<u64>().unwrap())
			.collect::<Vec<_>>();
		let operation: Vec<&str> = lines.next().unwrap()[19..].split(' ').collect();
		let test: u64 = lines.next().unwrap()[21..].parse().unwrap();
		let throw_true: usize = lines.next().unwrap()[29..].parse().unwrap();
		let throw_false: usize = lines.next().unwrap()[30..].parse().unwrap();

		let (op, right) = (operation[1], operation[2]);
		let operation = match (op, right) {
			(_, "old") => Operation::Square,
			("+", x) => Operation::Add(x.parse().unwrap()),
			("*", x) => Operation::Multiply(x.parse().unwrap()),
			_ => unimplemented!(),
		};

		monkeys.push(Monkey {
			items,
			operation,
			test,
			throw_true,
			throw_false,
			inspected: 0,
		});
	}

	monkeys
}

fn monkey_business(monkeys: &[Monkey]) -> usize {
	monkeys.iter().map(|m| m.inspected).sorted_by(|a, b| b.cmp(a)).take(2).product()
}

pub fn part1(input: &[Monkey]) -> usize {
	let mut monkeys = input.to_vec();

	for _ in 0..20 {
		for i in 0..monkeys.len() {
			let items: Vec<u64> = monkeys[i].items.iter().map(|old| monkeys[i].operation.compute(old)).collect();
			for item in items.iter() {
				let worry = item / 3;
				let id = if worry % monkeys[i].test == 0 {
					monkeys[i].throw_true
				} else {
					monkeys[i].throw_false
				};
				monkeys[id].items.push(worry);
			}
			monkeys[i].inspected += monkeys[i].items.len();
			monkeys[i].items.clear();
		}
	}

	monkey_business(&monkeys)
}

pub fn part2(input: &[Monkey]) -> usize {
	let global_modulo: u64 = input.iter().map(|m| m.test).product();
	let mut monkeys = input.to_vec();

	for _ in 0..10000 {
		for i in 0..monkeys.len() {
			let items: Vec<u64> = monkeys[i].items.iter().map(|old| monkeys[i].operation.compute(old)).collect();
			for item in items.iter() {
				let worry = item % global_modulo;
				let id = if worry % monkeys[i].test == 0 {
					monkeys[i].throw_true
				} else {
					monkeys[i].throw_false
				};

				monkeys[id].items.push(worry);
			}
			monkeys[i].inspected += monkeys[i].items.len();
			monkeys[i].items.clear();
		}
	}

	monkey_business(&monkeys)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 10605);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 2713310158);
	}
}
