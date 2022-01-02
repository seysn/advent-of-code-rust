use crate::run_day;

mod d01;
mod d02;
mod d03;
mod d04;

pub fn run(day: u8) {
    match day {
        1 => run_day!(y2015, d01),
        2 => run_day!(y2015, d02),
        3 => run_day!(y2015, d03),
        4 => run_day!(y2015, d04),
        _ => unimplemented!()
    }
}

pub fn run_all() {
    (1..=4).for_each(run)
}