use std::collections::VecDeque;

#[derive(Debug, Default, Clone)]
pub struct Boss {
	hit_points: u32,
	damage: u32,
	poison: u32,
}

pub fn parse_input(input: &str) -> Boss {
	let mut boss = Boss::default();
	for l in input.lines() {
		if let Some(s) = l.strip_prefix("Hit Points: ") {
			boss.hit_points = s.parse().unwrap();
		}

		if let Some(s) = l.strip_prefix("Damage: ") {
			boss.damage = s.parse().unwrap();
		}
	}

	boss
}

#[derive(Debug, Clone)]
struct Wizard {
	hit_points: u32,
	mana: u32,
	mana_spent: u32,
	shield: u32,
	recharge: u32,
}

#[derive(Debug, Clone)]
enum Turn {
	Wizard,
	Boss,
}

impl Turn {
	fn reverse(&self) -> Self {
		match self {
			Self::Wizard => Self::Boss,
			Self::Boss => Self::Wizard,
		}
	}
}

#[derive(Debug, Clone)]
struct Combat {
	boss: Boss,
	wizard: Wizard,
	turn: Turn,

	depth: u32,
}

fn solve(wizard: Wizard, boss: Boss, hard: bool) -> u32 {
	let mut queue = VecDeque::new();
	queue.push_back(Combat {
		wizard,
		boss,
		turn: Turn::Wizard,
		depth: 0,
	});

	let mut min_spent = u32::MAX;
	while let Some(mut combat) = queue.pop_front() {
		if combat.wizard.mana_spent >= min_spent {
			// Better solution already found: Skip
			continue;
		}

		if hard {
			// Lose
			if combat.wizard.hit_points == 1 {
				continue;
			}

			combat.wizard.hit_points -= 1;
		}

		let armor = if combat.wizard.shield > 0 {
			combat.wizard.shield -= 1;
			7
		} else {
			0
		};

		if combat.boss.poison > 0 {
			if combat.boss.hit_points <= 3 {
				// Win
				min_spent = combat.wizard.mana_spent;
				continue;
			} else {
				combat.boss.hit_points -= 3;
				combat.boss.poison -= 1;
			}
		}

		if combat.wizard.recharge > 0 {
			combat.wizard.mana += 101;
			combat.wizard.recharge -= 1;
		}

		if let Turn::Wizard = combat.turn {
			if combat.wizard.mana < 53 {
				// Lose
				continue;
			}

			// cast magic missile
			if combat.wizard.mana >= 53 {
				let mut next = combat.clone();
				next.wizard.mana_spent += 53;
				if next.boss.hit_points <= 4 {
					// Win
					min_spent = next.wizard.mana_spent;
					continue;
				} else {
					next.wizard.mana -= 53;
					next.boss.hit_points -= 4;
					next.turn = combat.turn.reverse();
					next.depth = combat.depth + 1;
					queue.push_back(next);
				}
			}

			// cast drain
			if combat.wizard.mana >= 73 {
				let mut next = combat.clone();
				next.wizard.mana_spent += 73;
				if next.boss.hit_points <= 2 {
					// Win
					min_spent = next.wizard.mana_spent;
					continue;
				} else {
					next.wizard.mana -= 73;
					next.boss.hit_points -= 2;
					next.wizard.hit_points += 2;
					next.turn = combat.turn.reverse();
					next.depth = combat.depth + 1;
					queue.push_back(next);
				}
			}

			// cast shield
			if combat.wizard.mana >= 113 && combat.wizard.shield == 0 {
				let mut next = combat.clone();
				next.wizard.mana -= 113;
				next.wizard.mana_spent += 113;
				next.wizard.shield = 6;
				next.turn = combat.turn.reverse();
				next.depth = combat.depth + 1;
				queue.push_back(next);
			}

			// cast poison
			if combat.wizard.mana >= 173 && combat.boss.poison == 0 {
				let mut next = combat.clone();
				next.wizard.mana -= 173;
				next.wizard.mana_spent += 173;
				next.boss.poison = 6;
				next.turn = combat.turn.reverse();
				next.depth = combat.depth + 1;
				queue.push_back(next);
			}

			// cast recharge
			if combat.wizard.mana >= 229 && combat.wizard.recharge == 0 {
				let mut next = combat.clone();
				next.wizard.mana -= 229;
				next.wizard.mana_spent += 229;
				next.wizard.recharge = 5;
				next.turn = combat.turn.reverse();
				next.depth = combat.depth + 1;
				queue.push_back(next);
			}
		} else {
			let dmg = if armor >= combat.boss.damage {
				1
			} else {
				combat.boss.damage - armor
			};

			if combat.wizard.hit_points <= dmg {
				// Lose
				continue;
			}

			let mut next = combat.clone();
			next.wizard.hit_points -= dmg;
			next.turn = combat.turn.reverse();
			next.depth = combat.depth + 1;
			queue.push_back(next);
		}
	}

	min_spent
}

pub fn part1(input: &Boss) -> u32 {
	solve(
		Wizard {
			hit_points: 50,
			mana: 500,
			mana_spent: 0,
			shield: 0,
			recharge: 0,
		},
		input.clone(),
		false,
	)
}

pub fn part2(input: &Boss) -> u32 {
	solve(
		Wizard {
			hit_points: 50,
			mana: 500,
			mana_spent: 0,
			shield: 0,
			recharge: 0,
		},
		input.clone(),
		true,
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(
			solve(
				Wizard {
					hit_points: 10,
					mana: 250,
					mana_spent: 0,
					shield: 0,
					recharge: 0,
				},
				Boss {
					hit_points: 14,
					damage: 8,
					poison: 0,
				},
				false
			),
			641
		);
	}
}
