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
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;

pub fn run(day: u8) {
	match day {
		1 => run_day!(y2023, d01),
		2 => run_day!(y2023, d02),
		3 => run_day!(y2023, d03),
		4 => run_day!(y2023, d04),
		5 => run_day!(y2023, d05),
		6 => run_day!(y2023, d06),
		7 => run_day!(y2023, d07),
		8 => run_day!(y2023, d08),
		9 => run_day!(y2023, d09),
		10 => run_day!(y2023, d10),
		11 => run_day!(y2023, d11),
		12 => run_day!(y2023, d12),
		13 => run_day!(y2023, d13),
		14 => run_day!(y2023, d14),
		15 => run_day!(y2023, d15),
		16 => run_day!(y2023, d16),
		17 => run_day!(y2023, d17),
		18 => run_day!(y2023, d18),
		19 => run_day!(y2023, d19),
		20 => run_day!(y2023, d20),
		21 => run_day!(y2023, d21),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	(1..=21).for_each(run)
}
