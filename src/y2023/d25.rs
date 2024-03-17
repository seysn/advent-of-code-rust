use itertools::Itertools;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone)]
pub struct Graph<'a> {
	wires: HashMap<&'a str, HashSet<&'a str>>,
	components: Vec<&'a str>,
}

impl<'a, 'b> Graph<'a> {
	fn remove_wire(&mut self, a: &'a str, b: &'a str) {
		self.wires.entry(a).and_modify(|e| {
			e.remove(b);
		});
		self.wires.entry(b).and_modify(|e| {
			e.remove(a);
		});
	}

	fn count_group(&self, start: &str) -> usize {
		let mut queue = VecDeque::new();
		let mut seen = HashSet::new();
		queue.push_back(start);

		while let Some(node) = queue.pop_front() {
			for next in self.wires.get(node).unwrap() {
				if !seen.contains(next) {
					queue.push_back(next);
					seen.insert(next);
				}
			}
		}

		seen.len()
	}

	fn dijkstra(&self, start: &'b str, end: &'b str) -> Vec<&'b str>
	where
		'a: 'b,
	{
		let mut queue = VecDeque::new();
		let mut came_from = HashMap::new();
		queue.push_back(start);

		while let Some(node) = queue.pop_front() {
			if node == end {
				break;
			}

			for next in self.wires.get(node).unwrap() {
				if !came_from.contains_key(next) {
					queue.push_back(next);
					came_from.insert(next, node);
				}
			}
		}

		let mut path = vec![end];
		let mut current = end;
		loop {
			current = came_from.get(&current).unwrap();
			path.push(current);
			if current == start {
				break;
			}
		}

		path
	}
}

pub fn parse_input(input: &str) -> Graph {
	let mut wires: HashMap<&str, HashSet<&str>> = HashMap::new();

	for line in input.lines() {
		let mut splited = line.split(": ");
		let k = splited.next().unwrap();
		let v = splited.next().unwrap().split(' ').collect::<HashSet<&str>>();
		wires.insert(k, v);
	}

	for (k, set) in wires.clone() {
		for v in set {
			wires.entry(v).or_default().insert(k);
		}
	}

	let components = wires.keys().cloned().collect();

	Graph { wires, components }
}

#[allow(unused_variables)]
pub fn part1(input: &Graph) -> usize {
	let mut counter: HashMap<&str, usize> = HashMap::new();
	let mut rng = &mut rand::thread_rng();
	for _ in 0..300 {
		let mut nodes = input.components.choose_multiple(&mut rng, 2);
		let start = *nodes.next().unwrap();
		let end = *nodes.next().unwrap();
		for node in input.dijkstra(start, end) {
			*counter.entry(node).or_default() += 1;
		}
	}

	let mut sorted_counter: Vec<(&&str, &usize)> = counter.iter().collect();
	sorted_counter.sort_by(|a, b| b.1.cmp(a.1));

	let mut graph = input.clone();
	for nodes in sorted_counter.iter().take(6).map(|(node, _)| node).combinations(2) {
		let mut it = nodes.iter();
		let a = it.next().unwrap();
		let b = it.next().unwrap();
		graph.remove_wire(a, b);
	}

	let mut results = HashSet::new();
	for (node, _) in sorted_counter.iter().take(6) {
		if results.len() == 2 {
			break;
		}

		results.insert(graph.count_group(node));
	}

	results.iter().product()
}

pub fn part2(_: &Graph) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 54);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 0);
	}
}
