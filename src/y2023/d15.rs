use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<String> {
	input.split(',').map(|step| step.to_string()).collect()
}

fn hash_step(s: &str) -> usize {
	s.chars().fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
}

pub fn part1(input: &[String]) -> usize {
	input.iter().map(|s| hash_step(s)).sum()
}

pub fn part2(input: &[String]) -> usize {
	let mut boxes: Vec<Vec<String>> = vec![Vec::new(); 256];
	let mut lens_box: HashMap<String, usize> = HashMap::new();
	let mut lens_focal: HashMap<String, usize> = HashMap::new();

	for elem in input {
		if elem.ends_with('-') {
			let label = &elem[..elem.len() - 1];
			if let Some(idx) = lens_box.remove(label) {
				lens_focal.remove(label);
				boxes[idx].retain(|l| l != label);
			}
		} else {
			let mut splited = elem.split('=');
			let label = splited.next().unwrap().to_string();
			let focal = splited.next().unwrap().parse().unwrap();
			let box_id = hash_step(&label);

			if lens_box.contains_key(&label) {
				lens_focal.insert(label, focal);
			} else {
				boxes[box_id].push(label.clone());
				lens_box.insert(label.clone(), box_id);
				lens_focal.insert(label, focal);
			}
		}
	}

	let mut res = 0;
	for (i, b) in boxes.iter().enumerate() {
		for (j, label) in b.iter().enumerate() {
			res += (i + 1) * (j + 1) * lens_focal.get(label).unwrap();
		}
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

	#[test]
	fn example_part1() {
		assert_eq!(hash_step("HASH"), 52);
		assert_eq!(part1(&parse_input(EXAMPLE)), 1320);
	}

	#[test]
	fn example_part2() {
		assert_eq!(hash_step("rn"), 0);
		assert_eq!(hash_step("qp"), 1);
		assert_eq!(hash_step("pc"), 3);
		assert_eq!(part2(&parse_input(EXAMPLE)), 145);
	}
}
