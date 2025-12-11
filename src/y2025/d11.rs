use std::collections::HashMap;

pub fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
	input
		.lines()
		.map(|line| {
			let (key, values) = line.split_once(':').unwrap();

			(key.to_owned(), values.split_whitespace().map(String::from).collect())
		})
		.collect()
}

fn dfs<'a>(from: &'a str, to: &str, graph: &'a HashMap<String, Vec<String>>, cache: &mut HashMap<&'a str, u64>) -> u64 {
	if from == to {
		return 1;
	}

	if let Some(v) = cache.get(from) {
		return *v;
	}

	let Some(children) = graph.get(from) else {
		return 0;
	};

	let mut sum = 0;
	for next in children {
		sum += dfs(next, to, graph, cache);
	}

	cache.insert(from, sum);
	sum
}

pub fn part1(input: &HashMap<String, Vec<String>>) -> u64 {
	dfs("you", "out", input, &mut HashMap::new())
}

pub fn part2(input: &HashMap<String, Vec<String>>) -> u64 {
	let a = dfs("svr", "fft", input, &mut HashMap::new());
	let b = dfs("fft", "dac", input, &mut HashMap::new());
	let c = dfs("dac", "out", input, &mut HashMap::new());

	let d = dfs("svr", "dac", input, &mut HashMap::new());
	let e = dfs("dac", "fft", input, &mut HashMap::new());
	let f = dfs("fft", "out", input, &mut HashMap::new());

	// One path is correct, the other one will return 0
	(a * b * c) + (d * e * f)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

	const EXAMPLE2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 5);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE2)), 2);
	}
}
