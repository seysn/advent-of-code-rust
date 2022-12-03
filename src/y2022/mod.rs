use crate::run_day;

mod d01;
mod d02;
mod d03;

pub fn run(day: u8) {
	match day {
		1 => run_day!(y2022, d01),
		2 => run_day!(y2022, d02),
		3 => run_day!(y2022, d03),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	(1..=3).for_each(run)
}
