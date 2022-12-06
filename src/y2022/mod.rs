use crate::run_day;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;

pub fn run(day: u8) {
	match day {
		1 => run_day!(y2022, d01),
		2 => run_day!(y2022, d02),
		3 => run_day!(y2022, d03),
		4 => run_day!(y2022, d04),
		5 => run_day!(y2022, d05),
		6 => run_day!(y2022, d06),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	(1..=6).for_each(run)
}
