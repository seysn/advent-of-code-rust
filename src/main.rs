use clap::Parser;

use advent_of_code::{run, run_all, run_all_year};

#[derive(Parser)]
struct Args {
	#[clap(short, long)]
	year: Option<u16>,

	#[clap(short, long)]
	day: Option<u8>,
}

fn main() {
	let args = Args::parse();

	match (args.year, args.day) {
		// Running specific year and day
		(Some(y), Some(d)) => run(y, d),
		// Running all days of a year
		(Some(y), _) => run_all_year(y),
		// Running all days of last year
		(_, Some(d)) => run(2021, d),
		// Running all days of all years
		(_, _) => run_all(),
	}
}
