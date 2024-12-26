use std::{
	collections::{HashMap, HashSet},
	hash::Hash,
};

pub fn parse_input(input: &str) -> Vec<(String, String)> {
	input
		.lines()
		.map(|l| {
			let (a, b) = l.split_once('-').unwrap();
			(a.to_owned(), b.to_owned())
		})
		.collect()
}

fn insert_order(a: String, b: String, c: String, set: &mut HashSet<(String, String, String)>) {
	if a < b && b < c && a < c {
		set.insert((a, b, c));
	} else if a < c && c < b && a < b {
		set.insert((a, c, b));
	} else if b < c && c < a && b < a {
		set.insert((b, c, a));
	} else if c < b && b < a && c < a {
		set.insert((c, b, a));
	} else if b < a && a < c && b < c {
		set.insert((b, a, c));
	} else if c < a && a < b && c < b {
		set.insert((c, a, b));
	}
}

pub fn part1(input: &[(String, String)]) -> usize {
	let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
	for connection in input {
		graph.entry(connection.0.clone()).or_default().insert(connection.1.clone());
		graph.entry(connection.1.clone()).or_default().insert(connection.0.clone());
	}

	let mut interconnected: HashSet<(String, String, String)> = HashSet::new();
	for (a, connections_a) in &graph {
		for b in connections_a {
			let connections_b = graph.get(b).unwrap();
			for c in connections_a.intersection(connections_b) {
				if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
					insert_order(a.clone(), b.clone(), c.clone(), &mut interconnected);
				}
			}
		}
	}

	interconnected.len()
}

#[derive(Debug)]
struct Clique(HashSet<String>);

impl PartialEq for Clique {
	fn eq(&self, other: &Clique) -> bool {
		self.0 == other.0
	}
}

impl Eq for Clique {}

impl Hash for Clique {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		let mut a: Vec<&String> = self.0.iter().collect();
		a.sort();
		for s in a.iter() {
			s.hash(state);
		}
	}
}

fn bron_kerbosch(
	r: HashSet<String>,
	mut p: HashSet<String>,
	mut x: HashSet<String>,
	graph: &HashMap<String, HashSet<String>>,
) -> HashSet<Clique> {
	let mut cliques: HashSet<Clique> = HashSet::new();
	if p.is_empty() && x.is_empty() {
		cliques.insert(Clique(r.clone()));
	}

	for v in p.clone().iter() {
		let g = graph.get(v).unwrap();

		let mut new_r = r.clone();
		new_r.insert(v.clone());

		let mut new_p = p.clone();
		new_p.retain(|s| g.contains(s));

		let mut new_x = x.clone();
		new_x.retain(|s| g.contains(s));

		for new_clique in bron_kerbosch(new_r, new_p, new_x, graph) {
			cliques.insert(new_clique);
		}

		p.remove(v);
		x.insert(v.clone());
	}

	cliques
}

pub fn part2(input: &[(String, String)]) -> String {
	let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
	for connection in input {
		graph.entry(connection.0.clone()).or_default().insert(connection.1.clone());
		graph.entry(connection.1.clone()).or_default().insert(connection.0.clone());
	}

	let vertices: HashSet<String> = graph.keys().cloned().collect();
	let cliques = bron_kerbosch(HashSet::new(), vertices, HashSet::new(), &graph);
	let max = cliques.iter().map(|c| c.0.len()).max().unwrap();
	let mut max_clique: Vec<String> = cliques.iter().find(|c| c.0.len() == max).unwrap().0.iter().cloned().collect();
	max_clique.sort();

	max_clique.join(",")
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 7);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), "co,de,ka,ta".to_owned());
	}
}
