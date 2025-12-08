use std::collections::{HashMap, HashSet, VecDeque};

use crate::collections::Point3D;

pub fn parse_input(input: &str) -> Vec<Point3D> {
	input
		.lines()
		.map(|l| {
			let mut splited = l.split(',');
			let x = splited.next().unwrap().parse().unwrap();
			let y = splited.next().unwrap().parse().unwrap();
			let z = splited.next().unwrap().parse().unwrap();

			Point3D(x, y, z)
		})
		.collect()
}

#[derive(Debug)]
struct Pair(Point3D, Point3D);

fn solve1(input: &[Point3D], wires: usize) -> u64 {
	let mut distances = Vec::new();
	for (i, p0) in input.iter().enumerate() {
		for p1 in &input[i + 1..] {
			distances.push((Pair(*p0, *p1), p0.euclidian_distance(p1)));
		}
	}

	distances.sort_by(|a, b| a.1.cmp(&b.1));

	let mut connections: HashMap<Point3D, Vec<Point3D>> = HashMap::new();
	for (pair, _) in &distances[..wires] {
		connections.entry(pair.0).or_default().push(pair.1);
		connections.entry(pair.1).or_default().push(pair.0);
	}

	let mut circuits = Vec::new();
	while !connections.is_empty() {
		let mut circuit = HashSet::new();
		let mut queue = VecDeque::new();

		let p = connections.keys().next().unwrap();
		queue.push_back(*p);
		circuit.insert(*p);

		while let Some(head) = queue.pop_front() {
			if let Some(points) = connections.remove(&head) {
				for p in points {
					circuit.insert(p);
					queue.push_back(p);
				}
			}
		}

		circuits.push(circuit.len());
	}

	circuits.sort_by(|a, b| b.cmp(a));

	circuits.iter().take(3).map(|v| *v as u64).product()
}

pub fn part1(input: &[Point3D]) -> u64 {
	solve1(input, 1000)
}

pub fn part2(input: &[Point3D]) -> u64 {
	let mut distances = Vec::new();
	for (i, p0) in input.iter().enumerate() {
		for p1 in &input[i + 1..] {
			distances.push((Pair(*p0, *p1), p0.euclidian_distance(p1)));
		}
	}

	distances.sort_by(|a, b| a.1.cmp(&b.1));

	let mut connected = HashSet::new();
	for (pair, _) in &distances {
		connected.insert(pair.0);
		connected.insert(pair.1);

		if connected.len() == input.len() {
			return pair.0.0 as u64 * pair.1.0 as u64;
		}
	}

	unreachable!()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

	#[test]
	fn example_part1() {
		assert_eq!(solve1(&parse_input(EXAMPLE), 10), 40);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 25272);
	}
}
