mod problem;
pub(crate) use problem::*;
mod days;

use std::fs;
use clap::{Parser, ValueHint};
use days::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name="number")]
    days: Vec<u8>,

    #[arg(short, long, value_hint=ValueHint::FilePath, help="Override default input file", value_name="file", conflicts_with="all_days")]
    input: Option<String>,

    #[arg(long, overrides_with="day")]
    all_days: bool
}

fn main() {
    let args = Args::parse();
    let numerical_days = if args.all_days { (1..=2).collect() } else { args.days };

    numerical_days.iter().for_each(|day| {
        let problem: Box<dyn Problem> = match day {
            1 => Box::new(Day1),
            2 => Box::new(Day2),
            _ => unimplemented!(),
        };

        let path = args.input.clone().unwrap_or(format!("inputs/day{day:02}.txt"));

        let input = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Could not find file at {path}"));

        let part_one = problem.part_one(&input);
        let part_two = problem.part_two(&input);

        println!("Day {day}");
        println!("--------");
        println!("Part One: {part_one}");
        println!("Part Two: {part_two}");
    });
}
