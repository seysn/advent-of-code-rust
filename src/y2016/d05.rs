//! Doesn't save me that much time to use a threaded solution but it is kind of cool

use std::{
	collections::HashMap,
	sync::{
		atomic::AtomicBool,
		mpsc::{self, Sender},
		Arc,
	},
};

const THREAD_COUNT: u32 = 4;

pub fn parse_input(input: &str) -> String {
	input.to_owned()
}

#[derive(Debug)]
struct Answer {
	index: u32,
	character: char,
}

fn solve(input: &str, start: u32, sender: Sender<Answer>, alive: Arc<AtomicBool>) {
	for i in (start..u32::MAX).step_by(THREAD_COUNT as usize) {
		if !alive.load(std::sync::atomic::Ordering::Relaxed) {
			break;
		}

		let digest = format!("{}{}", input, i);
		let result = format!("{:x}", md5::compute(digest.as_bytes()));

		if let Some(s) = result.strip_prefix("00000") {
			sender
				.send(Answer {
					index: i,
					character: s.chars().next().unwrap(),
				})
				.unwrap();
		}
	}
}

pub fn part1(input: &str) -> String {
	let (sender, receiver) = mpsc::channel();
	let mut answers = Vec::new();
	let alive = Arc::new(AtomicBool::new(true));
	let mut handles = Vec::new();

	for start in 0..THREAD_COUNT {
		let input = input.to_owned();
		let sender = sender.clone();
		let alive = alive.clone();
		let handle = std::thread::spawn(move || {
			solve(&input, start, sender, alive);
		});
		handles.push(handle);
	}

	loop {
		let ans = receiver.recv().unwrap();
		answers.push(ans);

		if answers.len() >= 8 {
			alive.store(false, std::sync::atomic::Ordering::Relaxed);
			answers.sort_by(|a, b| a.index.cmp(&b.index));

			while let Some(handle) = handles.pop() {
				handle.join().unwrap();
			}
			break;
		}
	}

	let mut res = String::new();
	for ans in answers {
		res.push(ans.character);
	}
	res
}

#[derive(Debug, PartialEq)]
struct Answer2 {
	index: u32,
	position: usize,
	character: char,
}

fn solve2(input: &str, start: u32, sender: Sender<Answer2>, alive: Arc<AtomicBool>) {
	for i in (start..u32::MAX).step_by(THREAD_COUNT as usize) {
		if !alive.load(std::sync::atomic::Ordering::Relaxed) {
			break;
		}

		let digest = format!("{}{}", input, i);
		let result = format!("{:x}", md5::compute(digest.as_bytes()));

		if let Some(s) = result.strip_prefix("00000") {
			let position = s.chars().next().unwrap();
			if ('0'..='7').contains(&position) {
				sender
					.send(Answer2 {
						index: i,
						position: position as usize - 48,
						character: s.chars().nth(1).unwrap(),
					})
					.unwrap();
			}
		}
	}
}

pub fn part2(input: &str) -> String {
	let (sender, receiver) = mpsc::channel();
	let mut answers: HashMap<usize, Answer2> = HashMap::new();
	let alive = Arc::new(AtomicBool::new(true));
	let mut handles = Vec::new();

	for start in 0..THREAD_COUNT {
		let input = input.to_owned();
		let sender = sender.clone();
		let alive = alive.clone();
		let handle = std::thread::spawn(move || {
			solve2(&input, start, sender, alive);
		});
		handles.push(handle);
	}

	loop {
		let ans = receiver.recv().unwrap();
		if let Some(old) = answers.get_mut(&ans.position) {
			if ans.index < old.index {
				old.index = ans.index;
				old.character = ans.character;
			}
		} else {
			answers.insert(ans.position, ans);
		}

		if answers.len() >= 8 {
			alive.store(false, std::sync::atomic::Ordering::Relaxed);

			while let Some(handle) = handles.pop() {
				handle.join().unwrap();
			}
			break;
		}
	}

	let mut res = ['_'; 8];
	for (_, ans) in answers {
		res[ans.position] = ans.character;
	}
	res.iter().collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "abc";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), "18f47a30".to_owned());
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), "05ace8e3".to_owned());
	}
}
