use crate::run_day;

mod intcode;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;

pub fn run(day: u8) {
	match day {
		1 => run_day!(y2019, d01),
		2 => run_day!(y2019, d02),
		3 => run_day!(y2019, d03),
		4 => run_day!(y2019, d04),
		5 => run_day!(y2019, d05),
		6 => run_day!(y2019, d06),
		7 => run_day!(y2019, d07),
		8 => run_day!(y2019, d08),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	(1..=8).for_each(run)
}
