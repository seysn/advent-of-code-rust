use crate::run_day;

mod d01;

pub fn run(day: u8) {
	match day {
		1 => run_day!(y2019, d01),
		_ => unimplemented!(),
	}
}

pub fn run_all() {
	(1..=1).for_each(run)
}
