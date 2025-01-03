use std::sync::LazyLock;

#[derive(Debug, Default)]
pub struct Boss {
	hit_points: u32,
	damage: u32,
	armor: u32,
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

		if let Some(s) = l.strip_prefix("Armor: ") {
			boss.armor = s.parse().unwrap();
		}
	}

	boss
}

#[derive(Debug)]
struct Value {
	value: u32,
	price: u32,
	rings: u32,
}

static DAMAGE: LazyLock<Vec<Value>> = LazyLock::new(|| {
	const WEAPONS: [(u32, u32); 5] = [(4, 8), (5, 10), (6, 25), (7, 40), (8, 74)];
	const RINGS_DMG: [(u32, u32); 3] = [(1, 25), (2, 50), (3, 100)];

	let mut damage: Vec<Value> = Vec::new();
	for (dmg, price) in WEAPONS {
		damage.push(Value {
			value: dmg,
			price,
			rings: 0,
		});

		for (ring, pprice) in RINGS_DMG {
			damage.push(Value {
				value: dmg + ring,
				price: price + pprice,
				rings: 1,
			});
		}

		damage.push(Value {
			value: dmg + RINGS_DMG[0].0 + RINGS_DMG[1].0,
			price: price + RINGS_DMG[0].1 + RINGS_DMG[1].1,
			rings: 2,
		});
		damage.push(Value {
			value: dmg + RINGS_DMG[1].0 + RINGS_DMG[2].0,
			price: price + RINGS_DMG[1].1 + RINGS_DMG[2].1,
			rings: 2,
		});
		damage.push(Value {
			value: dmg + RINGS_DMG[0].0 + RINGS_DMG[2].0,
			price: price + RINGS_DMG[0].1 + RINGS_DMG[2].1,
			rings: 2,
		});
	}

	damage
});

static ARMOR: LazyLock<Vec<Value>> = LazyLock::new(|| {
	const ARMORS: [(u32, u32); 6] = [(0, 0), (1, 13), (2, 31), (3, 53), (4, 75), (5, 102)];
	const RINGS_ARM: [(u32, u32); 3] = [(1, 20), (2, 40), (3, 80)];

	let mut armor: Vec<Value> = Vec::new();
	for (arm, price) in ARMORS {
		armor.push(Value {
			value: arm,
			price,
			rings: 0,
		});

		for (ring, pprice) in RINGS_ARM {
			armor.push(Value {
				value: arm + ring,
				price: price + pprice,
				rings: 1,
			});
		}

		armor.push(Value {
			value: arm + RINGS_ARM[0].0 + RINGS_ARM[1].0,
			price: price + RINGS_ARM[0].1 + RINGS_ARM[1].1,
			rings: 2,
		});
		armor.push(Value {
			value: arm + RINGS_ARM[1].0 + RINGS_ARM[2].0,
			price: price + RINGS_ARM[1].1 + RINGS_ARM[2].1,
			rings: 2,
		});
		armor.push(Value {
			value: arm + RINGS_ARM[0].0 + RINGS_ARM[2].0,
			price: price + RINGS_ARM[0].1 + RINGS_ARM[2].1,
			rings: 2,
		});
	}

	armor
});

pub fn part1(input: &Boss) -> u32 {
	let damage = &*DAMAGE;
	let armor = &*ARMOR;

	let mut res = Vec::new();
	for dmg in damage {
		let hit_per_turn = if input.armor >= dmg.value { 1 } else { dmg.value - input.armor };
		let turns = if input.hit_points % hit_per_turn == 0 {
			input.hit_points / hit_per_turn
		} else {
			input.hit_points / hit_per_turn + 1
		};

		for arm in armor {
			let boss_hit_per_turn = if arm.value >= input.damage { 1 } else { input.damage - arm.value };
			let boss_turns = if 100 % boss_hit_per_turn == 0 {
				100 / boss_hit_per_turn
			} else {
				100 / boss_hit_per_turn + 1
			};

			if boss_turns < turns {
				continue;
			}

			if dmg.rings + arm.rings > 2 {
				continue;
			}

			res.push(dmg.price + arm.price);
		}
	}

	*res.iter().min().unwrap()
}

pub fn part2(input: &Boss) -> u32 {
	let damage = &*DAMAGE;
	let armor = &*ARMOR;

	let mut res = Vec::new();
	for dmg in damage {
		let hit_per_turn = if input.armor >= dmg.value { 1 } else { dmg.value - input.armor };
		let turns = if input.hit_points % hit_per_turn == 0 {
			input.hit_points / hit_per_turn
		} else {
			input.hit_points / hit_per_turn + 1
		};

		for arm in armor {
			let boss_hit_per_turn = if arm.value >= input.damage { 1 } else { input.damage - arm.value };
			let boss_turns = if 100 % boss_hit_per_turn == 0 {
				100 / boss_hit_per_turn
			} else {
				100 / boss_hit_per_turn + 1
			};

			if boss_turns >= turns {
				continue;
			}

			if dmg.rings + arm.rings > 2 {
				continue;
			}

			res.push(dmg.price + arm.price);
		}
	}

	*res.iter().max().unwrap()
}
