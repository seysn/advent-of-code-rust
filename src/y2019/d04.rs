use std::ops::RangeInclusive;

pub fn parse_input(input: &str) -> RangeInclusive<usize> {
	assert_eq!(input.len(), 13);
	let a = input[..6].parse().unwrap();
	let b = input[7..].parse().unwrap();
	a..=b
}

fn digits(n: usize) -> [usize; 6] {
	let mut res = [0; 6];

	let mut value = n;
	for i in 0..6 {
		res[5 - i] = value % 10;
		value /= 10;
	}

	res
}

fn is_password(digits: &[usize; 6]) -> bool {
	let mut double = false;
	for (a, b) in digits.iter().zip(digits.iter().skip(1)) {
		match a.cmp(b) {
			std::cmp::Ordering::Less => (),
			std::cmp::Ordering::Equal => double = true,
			std::cmp::Ordering::Greater => return false,
		}
	}

	double
}

fn is_real_password(digits: &[usize; 6]) -> bool {
	let mut double = false;
	let mut consecutive = 1;
	for (a, b) in digits.iter().zip(digits.iter().skip(1)) {
		if a == b {
			consecutive += 1;
		} else if a != b {
			if a > b {
				return false;
			}
			if consecutive == 2 {
				double = true;
			}
			consecutive = 1;
		}
	}

	double || consecutive == 2
}

pub fn part1(input: &RangeInclusive<usize>) -> usize {
	input.clone().filter(|&n| is_password(&digits(n))).count()
}

pub fn part2(input: &RangeInclusive<usize>) -> usize {
	input.clone().filter(|&n| is_real_password(&digits(n))).count()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert!(is_password(&digits(111111)));
		assert!(!is_password(&digits(223450)));
		assert!(!is_password(&digits(123789)));
	}

	#[test]
	fn example_part2() {
		assert!(!is_real_password(&digits(223450)));
		assert!(is_real_password(&digits(112233)));
		assert!(!is_real_password(&digits(123444)));
		assert!(is_real_password(&digits(111122)));
		assert!(is_real_password(&digits(111233)));
	}
}
