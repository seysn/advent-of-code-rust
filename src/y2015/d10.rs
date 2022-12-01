use std::fmt::Write as _;

pub fn parse_input(input: &str) -> String {
	String::from(input)
}

fn step(sequence: &str) -> String {
	let mut res = String::new();
	let mut prev = sequence.chars().next().unwrap();
	let mut cpt = 1;

	for c in sequence.chars().skip(1) {
		if prev == c {
			cpt += 1;
		} else {
			write!(res, "{}{}", cpt, prev).unwrap();
			cpt = 1;
			prev = c;
		}
	}

	write!(res, "{}{}", cpt, prev).unwrap();

	res
}

pub fn part1(input: &str) -> usize {
	let mut s = String::from(input);
	for _ in 0..40 {
		s = step(&s);
	}
	s.len()
}

pub fn part2(input: &str) -> usize {
	let mut s = String::from(input);
	for _ in 0..50 {
		s = step(&s);
	}
	s.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_step() {
		assert_eq!(step("1"), "11");
		assert_eq!(step("11"), "21");
		assert_eq!(step("21"), "1211");
		assert_eq!(step("1211"), "111221");
		assert_eq!(step("111221"), "312211");
	}
}
