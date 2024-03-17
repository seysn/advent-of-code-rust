use crate::run_day;

mod intcode;

mod d01;
mod d02;

pub fn run(day: u8) {
	match day {
		1 => run_day!(y2019, d01),
		2 => run_day!(y2019, d02),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	(1..=2).for_each(run)
}
