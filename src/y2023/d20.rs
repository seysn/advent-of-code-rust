use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
pub struct Module {
	name: String,
	ty: ModuleType,
	inputs: Vec<String>,
	outputs: Vec<String>,
}

impl Module {
	fn new(name: &str, outputs: Vec<String>) -> Self {
		let (name, ty) = if let Some(stripped) = name.strip_prefix('%') {
			(stripped.to_string(), ModuleType::FlipFlop)
		} else if let Some(stripped) = name.strip_prefix('&') {
			(stripped.to_string(), ModuleType::Conjunction)
		} else {
			(name.to_string(), ModuleType::Broadcaster)
		};

		Module {
			name,
			ty,
			inputs: Vec::new(),
			outputs,
		}
	}
}

#[derive(Clone, Copy)]
enum ModuleType {
	FlipFlop,
	Broadcaster,
	Conjunction,
}

pub fn parse_input(input: &str) -> HashMap<String, Module> {
	let mut modules = HashMap::new();
	for line in input.lines() {
		let mut splited = line.split(" -> ");
		let name = splited.next().unwrap();
		let outputs = splited.next().unwrap().split(", ").map(String::from).collect();

		let module = Module::new(name, outputs);
		modules.insert(module.name.clone(), module);
	}

	for (name, module) in &modules.clone() {
		for output in &module.outputs {
			if let Some(out) = modules.get_mut(output) {
				out.inputs.push(name.clone());
			}
		}
	}

	modules
}

#[derive(Clone, Copy)]
enum PulseIntensity {
	Low,
	High,
}

#[derive(Clone, Copy)]
enum FlipFlopState {
	On,
	Off,
}

impl FlipFlopState {
	fn reverse(&self) -> Self {
		match self {
			FlipFlopState::On => FlipFlopState::Off,
			FlipFlopState::Off => FlipFlopState::On,
		}
	}
}

struct State {
	flipflop: HashMap<String, FlipFlopState>,
	conjunction: HashMap<String, HashMap<String, PulseIntensity>>,
}

struct Pulse {
	module: String,
	intensity: PulseIntensity,
	previous: String,
}

impl Default for Pulse {
	fn default() -> Self {
		Pulse {
			module: "broadcaster".to_string(),
			intensity: PulseIntensity::Low,
			previous: "button".to_string(),
		}
	}
}

fn push_button(input: &HashMap<String, Module>, pulses: &mut VecDeque<Pulse>, state: &mut State) -> (usize, usize) {
	let mut low_pulses = 0;
	let mut high_pulses = 0;

	pulses.push_back(Pulse::default());
	while let Some(Pulse {
		module,
		intensity,
		previous,
	}) = pulses.pop_front()
	{
		match intensity {
			PulseIntensity::Low => low_pulses += 1,
			PulseIntensity::High => high_pulses += 1,
		}

		let t = if let Some(m) = input.get(&module) {
			m.ty
		} else {
			continue;
		};
		let destinations: &[String] = &input.get(&module).unwrap().outputs;

		match t {
			ModuleType::Broadcaster => {
				for next in destinations {
					pulses.push_back(Pulse {
						module: next.clone(),
						intensity,
						previous: module.clone(),
					});
				}
			}
			ModuleType::FlipFlop => {
				if matches!(intensity, PulseIntensity::High) {
					continue;
				}

				let rev = state.flipflop.get(&module).unwrap().reverse();
				for next in destinations {
					pulses.push_back(Pulse {
						module: next.clone(),
						intensity: if let FlipFlopState::On = rev {
							PulseIntensity::High
						} else {
							PulseIntensity::Low
						},
						previous: module.clone(),
					});
				}
				state.flipflop.insert(module, rev);
			}
			ModuleType::Conjunction => {
				let mem = state.conjunction.get_mut(&module).unwrap();
				mem.insert(previous.clone(), intensity);
				let next_intensity = if mem.values().all(|p| matches!(p, PulseIntensity::High)) {
					PulseIntensity::Low
				} else {
					PulseIntensity::High
				};

				for next in destinations {
					pulses.push_back(Pulse {
						module: next.clone(),
						intensity: next_intensity,
						previous: module.clone(),
					});
				}
			}
		}
	}

	(low_pulses, high_pulses)
}

fn push_button2(input: &HashMap<String, Module>, cycle_ends: &[String], pulses: &mut VecDeque<Pulse>, state: &mut State) -> Option<String> {
	let mut res = None;

	pulses.push_back(Pulse::default());
	while let Some(Pulse {
		module,
		intensity,
		previous,
	}) = pulses.pop_front()
	{
		if cycle_ends.contains(&module) && matches!(intensity, PulseIntensity::Low) {
			res = Some(module.clone());
		}

		let t = if let Some(m) = input.get(&module) {
			m.ty
		} else {
			continue;
		};
		let destinations: &[String] = &input.get(&module).unwrap().outputs;

		match t {
			ModuleType::Broadcaster => {
				for next in destinations {
					pulses.push_back(Pulse {
						module: next.clone(),
						intensity,
						previous: module.clone(),
					});
				}
			}
			ModuleType::FlipFlop => {
				if matches!(intensity, PulseIntensity::High) {
					continue;
				}

				let rev = state.flipflop.get(&module).unwrap().reverse();
				for next in destinations {
					pulses.push_back(Pulse {
						module: next.clone(),
						intensity: if let FlipFlopState::On = rev {
							PulseIntensity::High
						} else {
							PulseIntensity::Low
						},
						previous: module.clone(),
					});
				}
				state.flipflop.insert(module, rev);
			}
			ModuleType::Conjunction => {
				let mem = state.conjunction.get_mut(&module).unwrap();
				mem.insert(previous.clone(), intensity);
				let next_intensity = if mem.values().all(|p| matches!(p, PulseIntensity::High)) {
					PulseIntensity::Low
				} else {
					PulseIntensity::High
				};

				for next in destinations {
					pulses.push_back(Pulse {
						module: next.clone(),
						intensity: next_intensity,
						previous: module.clone(),
					});
				}
			}
		}
	}

	res
}

impl State {
	fn new(input: &HashMap<String, Module>) -> State {
		let mut flipflop = HashMap::new();
		let mut conjunction = HashMap::new();

		for (name, module) in input {
			match module.ty {
				ModuleType::FlipFlop => {
					flipflop.insert(name.clone(), FlipFlopState::Off);
				}
				ModuleType::Conjunction => {
					conjunction.insert(
						name.clone(),
						module.inputs.iter().map(|m| (m.clone(), PulseIntensity::Low)).collect(),
					);
				}
				ModuleType::Broadcaster => {}
			}
		}

		State { flipflop, conjunction }
	}
}

pub fn part1(input: &HashMap<String, Module>) -> usize {
	let mut pulses = VecDeque::new();
	let mut state = State::new(input);

	let mut low_pulses = 0;
	let mut high_pulses = 0;

	for _ in 0..1000 {
		// println!("=== Button pushed ===");
		let (l, h) = push_button(input, &mut pulses, &mut state);
		low_pulses += l;
		high_pulses += h;
	}
	low_pulses * high_pulses
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

pub fn part2(input: &HashMap<String, Module>) -> usize {
	let mut last_conj = None;
	for (key, module) in input {
		if module.outputs.iter().any(|s| s == "rx") {
			last_conj = Some(key);
		}
	}

	let cycle_ends = if let Some(m) = last_conj {
		&input.get(m).unwrap().inputs
	} else {
		panic!("Invalid input")
	};

	let mut pulses = VecDeque::new();
	let mut state = State::new(input);
	let mut results = HashMap::new();

	for i in 1..usize::MAX {
		if let Some(m) = push_button2(input, cycle_ends, &mut pulses, &mut state) {
			if !results.contains_key(&m) {
				results.insert(m.clone(), i);
			}
			if results.len() == cycle_ends.len() {
				break;
			}
		}
	}
	dbg!(&results);

	let values: Vec<usize> = results.values().copied().collect();
	lcm(&values)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

	const EXAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 32000000);
		assert_eq!(part1(&parse_input(EXAMPLE2)), 11687500);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 0);
	}
}
