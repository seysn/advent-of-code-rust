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

pub fn run(day: u8) {
	match day {
		1 => run_day!(y2021, d01),
		2 => run_day!(y2021, d02),
		3 => run_day!(y2021, d03),
		4 => run_day!(y2021, d04),
		5 => run_day!(y2021, d05),
		6 => run_day!(y2021, d06),
		7 => run_day!(y2021, d07),
		8 => run_day!(y2021, d08),
		9 => run_day!(y2021, d09),
		10 => run_day!(y2021, d10),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	(1..=10).for_each(run)
}
