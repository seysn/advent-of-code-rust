mod utils;

mod y2015;
mod y2020;
mod y2021;
mod y2022;

#[macro_export]
macro_rules! run_day {
	($year:path, $day:path) => {{
		use $day::{parse_input, part1, part2};

		// Read and parse input
		let content = $crate::utils::get_input(
			$crate::utils::extract_integer(stringify!($year)).unwrap(),
			$crate::utils::extract_integer(stringify!($day)).unwrap(),
		);
		let input = parse_input(&content);

		// Running part 1
		let before = std::time::Instant::now();
		let part1_out = part1(&input);
		let part1_time = before.elapsed();

		// Running part 2
		let before = std::time::Instant::now();
		let part2_out = part2(&input);
		let part2_time = before.elapsed();

		println!(
			"{}: part1 = {} ({:.2?}), part2 = {} ({:.2?})",
			stringify!($day),
			part1_out,
			part1_time,
			part2_out,
			part2_time
		);
	}};
}

pub fn run(year: u16, day: u8) {
	match year {
		2015 => y2015::run(day),
		2020 => y2020::run(day),
		2021 => y2021::run(day),
		2022 => y2022::run(day),
		_ => unimplemented!(),
	}
}

pub fn run_all_year(year: u16) {
	match year {
		2015 => y2015::run_all(),
		2020 => y2020::run_all(),
		2021 => y2021::run_all(),
		2022 => y2022::run_all(),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	for year in [2015, 2020, 2021, 2022] {
		println!("YEAR {}:", year);
		run_all_year(year);
	}
}
