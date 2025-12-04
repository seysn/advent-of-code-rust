use std::collections::HashMap;

pub struct Row {
	record: Vec<char>,
	groups: Vec<usize>,
}

impl From<&str> for Row {
	fn from(value: &str) -> Self {
		let mut splited = value.split(' ');
		let record = splited.next().unwrap().chars().collect();
		let groups = splited.next().unwrap().split(',').map(|n| n.parse::<usize>().unwrap()).collect();

		Row { record, groups }
	}
}

pub fn parse_input(input: &str) -> Vec<Row> {
	input.lines().map(Row::from).collect()
}

fn generate(record: &[char], groups: &[usize], cache: &mut HashMap<String, usize>) -> usize {
	let record_s = format!("{}{:?}", String::from_iter(record.iter()), groups);
	if cache.contains_key(&record_s) {
		return *cache.get(&record_s).unwrap();
	}

	if groups.is_empty() {
		if record.contains(&'#') {
			cache.insert(record_s, 0);
			return 0;
		}
		cache.insert(record_s, 1);
		return 1;
	}

	if record.is_empty() {
		cache.insert(record_s, 0);
		return 0;
	}

	match record[0] {
		'.' => generate(&record[1..], groups, cache),
		'?' => {
			let mut r0 = record.to_vec();
			let mut r1 = record.to_vec();
			r0[0] = '.';
			r1[0] = '#';
			let res0 = generate(&r0, groups, cache);
			let res1 = generate(&r1, groups, cache);
			cache.insert(format!("{}{:?}", String::from_iter(r0.iter()), groups), res0);
			cache.insert(format!("{}{:?}", String::from_iter(r1.iter()), groups), res1);
			res0 + res1
		}
		'#' => {
			let n = record.iter().take_while(|&&c| c != '.').count();
			if n < groups[0] || (record.len() > groups[0] + 1 && record[groups[0]] == '#') {
				cache.insert(record_s, 0);
				return 0;
			}

			let mut r2 = record.to_vec();
			for c in r2.iter_mut().take(groups[0]) {
				*c = '#';
			}

			if r2.len() > groups[0] + 1 {
				r2[groups[0]] = '.';
			}

			if r2.len() > groups[0] + 1 {
				let res = generate(&r2[groups[0] + 1..], &groups[1..], cache);
				cache.insert(format!("{}{:?}", String::from_iter(&r2[groups[0] + 1..]), groups), res);
				res
			} else if groups.len() <= 1 && !r2.iter().skip(groups[0]).any(|&c| c == '#') {
				cache.insert(record_s, 1);
				1
			} else {
				cache.insert(record_s, 0);
				0
			}
		}
		_ => unreachable!(),
	}
}

pub fn part1(input: &[Row]) -> usize {
	let mut cache: HashMap<String, usize> = HashMap::new();
	input.iter().map(|row| generate(&row.record, &row.groups, &mut cache)).sum()
}

fn repeat_join(record: &[char]) -> Vec<char> {
	let mut res = String::new();

	for _ in 0..4 {
		res += &String::from_iter(record);
		res += "?";
	}
	res += &String::from_iter(record);

	res.chars().collect()
}

pub fn part2(input: &[Row]) -> usize {
	let mut res = 0;
	for row in input {
		let mut cache: HashMap<String, usize> = HashMap::new();
		res += generate(&repeat_join(&row.record), &row.groups.repeat(5), &mut cache);
	}
	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 21);
		assert_eq!(part1(&parse_input("?#?#???????????# 12,1")), 2);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 525152);
	}
}
