use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Input {
	wires: HashMap<String, bool>,
	gates: Vec<Gate>,
}

#[derive(Debug, Clone)]
pub enum Gate {
	And(String, String, String),
	Or(String, String, String),
	Xor(String, String, String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum GateOp {
	And,
	Or,
	Xor,
}

impl Gate {
	fn op(&self) -> GateOp {
		match self {
			Gate::And(_, _, _) => GateOp::And,
			Gate::Or(_, _, _) => GateOp::Or,
			Gate::Xor(_, _, _) => GateOp::Xor,
		}
	}
	fn left(&self) -> &str {
		match self {
			Gate::And(v, _, _) | Gate::Or(v, _, _) | Gate::Xor(v, _, _) => v,
		}
	}

	fn right(&self) -> &str {
		match self {
			Gate::And(_, v, _) | Gate::Or(_, v, _) | Gate::Xor(_, v, _) => v,
		}
	}

	fn output(&self) -> &str {
		match self {
			Gate::And(_, _, v) | Gate::Or(_, _, v) | Gate::Xor(_, _, v) => v,
		}
	}
}

pub fn parse_input(input: &str) -> Input {
	let (wires, gates) = input.split_once("\n\n").unwrap();

	Input {
		wires: wires
			.lines()
			.map(|l| {
				let (k, v) = l.split_once(": ").unwrap();
				(k.to_owned(), v.starts_with('1'))
			})
			.collect(),
		gates: gates
			.lines()
			.map(|l| {
				let v: Vec<&str> = l.split(' ').collect();

				match v[1] {
					"AND" => Gate::And(v[0].to_owned(), v[2].to_owned(), v[4].to_owned()),
					"OR" => Gate::Or(v[0].to_owned(), v[2].to_owned(), v[4].to_owned()),
					"XOR" => Gate::Xor(v[0].to_owned(), v[2].to_owned(), v[4].to_owned()),
					_ => unreachable!(),
				}
			})
			.collect(),
	}
}

fn run(mut wires: HashMap<String, bool>, gates: &[Gate]) -> u64 {
	let mut gates: VecDeque<Gate> = VecDeque::from_iter(gates.iter().cloned());

	while let Some(gate) = gates.pop_front() {
		if let (Some(a), Some(b)) = (wires.get(gate.left()), wires.get(gate.right())) {
			let res = match gate {
				Gate::And(_, _, _) => *a && *b,
				Gate::Or(_, _, _) => *a || *b,
				Gate::Xor(_, _, _) => *a ^ *b,
			};

			wires.insert(gate.output().to_owned(), res);
		} else {
			gates.push_back(gate);
		}
	}

	let mut res: u64 = 0;
	for (wire, b) in wires {
		if !b {
			continue;
		}

		if let Some(v) = wire.strip_prefix("z") {
			let bit: u64 = v.parse().unwrap();
			res |= 1 << bit;
		}
	}

	res
}

pub fn part1(input: &Input) -> u64 {
	run(input.wires.clone(), &input.gates)
}

/// Initially solved manually but wanted to get an automatic solution.
/// The idea is that instead of swapping, we can just check incorrect gates based on
/// simple rules described below.
/// Based on the solution :
/// https://www.reddit.com/r/adventofcode/comments/1hl698z/2024_day_24_solutions/m3kt1je/
pub fn part2(input: &Input) -> String {
	let mut swaped = HashSet::new();
	for gate in &input.gates {
		let left = gate.left();
		let right = gate.right();
		let out = gate.output();
		let op = gate.op();

		// The final adder gate has to be a Xor (except the last one)
		if out.starts_with("z") && op != GateOp::Xor && out != "z45" {
			swaped.insert(out.to_owned());
		}

		// Xor gates should have x/y/z input/output
		if op == GateOp::Xor
			&& !left.starts_with(['x', 'y', 'z'])
			&& !right.starts_with(['x', 'y', 'z'])
			&& !out.starts_with(['x', 'y', 'z'])
		{
			swaped.insert(out.to_owned());
		}

		// The gate after a And gate should be a Or gate
		if op == GateOp::And && left != "x00" && right != "x00" {
			for subgate in &input.gates {
				if (out == subgate.left() || out == subgate.right()) && subgate.op() != GateOp::Or {
					swaped.insert(out.to_owned());
				}
			}
		}

		// The gate after a Xor gate shouldn't be a Or gate
		if op == GateOp::Xor {
			for subgate in &input.gates {
				if (out == subgate.left() || out == subgate.right()) && subgate.op() == GateOp::Or {
					swaped.insert(out.to_owned());
				}
			}
		}
	}

	let mut res: Vec<String> = swaped.into_iter().collect();
	res.sort();

	res.join(",")
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 2024);
	}
}
