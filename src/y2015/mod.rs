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
mod d12;
mod d13;

pub fn run(day: u8) {
	match day {
		1 => run_day!(y2015, d01),
		2 => run_day!(y2015, d02),
		3 => run_day!(y2015, d03),
		4 => run_day!(y2015, d04),
		5 => run_day!(y2015, d05),
		6 => run_day!(y2015, d06),
		7 => run_day!(y2015, d07),
		8 => run_day!(y2015, d08),
		9 => run_day!(y2015, d09),
		10 => run_day!(y2015, d10),
		11 => run_day!(y2015, d11),
		12 => run_day!(y2015, d12),
		13 => run_day!(y2015, d13),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	(1..=13).for_each(run)
}
