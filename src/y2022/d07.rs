use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

pub struct Command {
	command: String,
	output: Vec<String>,
}

type Node = Rc<RefCell<Item>>;

// I regret making the distinction between a File and a Directory...
enum Item {
	File {
		size: u64,
	},
	Directory {
		name: String,
		children: BTreeMap<String, Node>,
		parent: Node,
	},
	Root,
}

pub fn parse_input(input: &str) -> Vec<Command> {
	let mut commands = Vec::new();

	let mut command = String::new();
	let mut output = Vec::new();
	for line in input.lines() {
		if let Some(cmd) = line.strip_prefix("$ ") {
			if !command.is_empty() {
				commands.push(Command {
					command,
					output: output.clone(),
				});
				output.clear();
			}
			command = String::from(cmd);
		} else {
			output.push(String::from(line))
		}
	}

	commands.push(Command {
		command,
		output: output.clone(),
	});

	commands
}

fn parse_commands(input: &[Command]) -> Node {
	let root = Rc::new(RefCell::new(Item::Directory {
		name: "/".to_string(),
		children: BTreeMap::new(),
		parent: Rc::new(RefCell::new(Item::Root)),
	}));
	let mut pwd = root.clone();

	for cmd in input {
		if let Some(path) = cmd.command.strip_prefix("cd ") {
			pwd = match path {
				"/" => root.clone(),
				".." => match &*pwd.borrow() {
					Item::Directory { parent, .. } => parent.clone(),
					_ => root.clone(),
				},
				_ => match &*pwd.borrow() {
					Item::Directory { children, .. } => children.get(path).unwrap().clone(),
					_ => unreachable!(),
				},
			}
		} else {
			for output in &cmd.output {
				if let Item::Directory { ref mut children, .. } = *pwd.borrow_mut() {
					if let Some(dir) = output.strip_prefix("dir ") {
						children.insert(
							dir.to_string(),
							Rc::new(RefCell::new(Item::Directory {
								name: dir.to_string(),
								children: BTreeMap::new(),
								parent: pwd.clone(),
							})),
						);
					} else {
						let (size, name) = output.split_once(' ').unwrap();
						children.insert(
							name.to_string(),
							Rc::new(RefCell::new(Item::File {
								size: size.parse().unwrap(),
							})),
						);
					}
				}
			}
		}
	}

	root
}

fn dir_size(dir: &Rc<RefCell<Item>>, dir_sums: &mut BTreeMap<String, u64>, parent_name: &String) -> u64 {
	if let Item::Directory { name, children, .. } = &*dir.borrow() {
		let mut sum = 0;
		for child in children.values() {
			match *child.borrow() {
				Item::Directory { .. } => sum += dir_size(child, dir_sums, name),
				Item::File { size } => sum += size,
				Item::Root => unreachable!(),
			};
		}

		dir_sums.insert(format!("{parent_name}.{name}"), sum);
		return sum;
	}
	unreachable!()
}

pub fn part1(input: &[Command]) -> u64 {
	let mut dir_sums: BTreeMap<String, u64> = BTreeMap::new();
	let root = parse_commands(input);
	dir_size(&root, &mut dir_sums, &"/".to_string());

	dir_sums.values().filter(|&&x| x < 100000).sum()
}

pub fn part2(input: &[Command]) -> u64 {
	let mut dir_sums: BTreeMap<String, u64> = BTreeMap::new();
	let root = parse_commands(input);
	let root_size = dir_size(&root, &mut dir_sums, &"/".to_string());
	let unused_space = 70000000 - root_size;

	*dir_sums.values().filter(|&&x| x > 30000000 - unused_space).min().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 95437);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 24933642);
	}
}
