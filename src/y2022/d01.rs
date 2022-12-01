pub fn parse_input(input: &str) -> Vec<i32> {
	input.lines().map(|l| if l.is_empty() {
		-1
	} else {
		l.parse().unwrap()
	}).collect()
}

pub fn part1(input: &[i32]) -> i32 {
	let mut most = 0;
	let mut tmp = 0;
	for &i in input {
		if i == -1 {
			if tmp > most {
				most = tmp;
			}
			tmp = 0;
		}
		tmp += i;
	}
	most + 1
}

pub fn part2(input: &[i32]) -> i32 {
	let mut lst = vec![];
	let mut tmp = 0;
	for &i in input {
		if i == -1 {
			lst.push(tmp + 1);
			tmp = 0;
		}
		tmp += i;
	}
	lst.push(tmp + 1);

	lst.sort_by(|a, b| b.cmp(a));
	dbg!(&lst);
	lst[0] + lst[1] + lst[2]
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 24000);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 45000);
	}
}
