use crate::run_day;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;

pub fn run(day: u8) {
	match day {
		1 => run_day!(y2024, d01),
		2 => run_day!(y2024, d02),
		3 => run_day!(y2024, d03),
		4 => run_day!(y2024, d04),
		5 => run_day!(y2024, d05),
		6 => run_day!(y2024, d06),
		7 => run_day!(y2024, d07),
		8 => run_day!(y2024, d08),
		9 => run_day!(y2024, d09),
		10 => run_day!(y2024, d10),
		11 => run_day!(y2024, d11),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	(1..=11).for_each(run)
}
