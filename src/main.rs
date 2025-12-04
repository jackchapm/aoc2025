mod problem;
pub(crate) use problem::*;
mod days;

use std::fs;
use std::time::Instant;
use clap::{Parser, builder::ArgPredicate, ValueHint};
use reqwest::blocking::Client;
use days::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name="number", overrides_with="all_days")]
    days: Vec<u8>,

    #[arg(short, long, value_hint=ValueHint::FilePath, help="Override default input file", value_name="file", conflicts_with="all_days")]
    input: Option<String>,

    #[arg(long, default_value="true", default_value_if("days", ArgPredicate::IsPresent, "false"))]
    all_days: bool,

    #[arg(long, help="Downloads missing input files", requires="session")]
    download: bool,

    #[arg(long, hide=true, env="AOC_SESSION")]
    session: Option<String>,
}

macro_rules! err {
    ($msg:literal) => {{
        eprintln!($msg);
        std::process::exit(1);
    }};
}

fn main() {
    let args = Args::parse();
    let numerical_days = if args.all_days { (1..=3).collect() } else { args.days };
    let session = args.session.unwrap_or(String::new());

    if args.download && !fs::exists("inputs/").unwrap_or(false) {
        fs::create_dir("inputs").unwrap_or_else(|e| err!("Could not create inputs folder\n{e}"))
    }

    numerical_days.iter().for_each(|day| {
        let problem: Box<dyn Problem> = match day {
            1 => Box::new(Day1),
            2 => Box::new(Day2),
            3 => Box::new(Day3),
            _ => unimplemented!(),
        };

        let path = args.input.clone().unwrap_or(format!("inputs/day{day:02}.txt"));
        let input = if fs::exists(&path).unwrap_or_else(|_| err!("Could not open file at {path}")) {
            fs::read_to_string(&path).unwrap()
        } else if args.download {
            let client = Client::new();
            let url = format!("https://adventofcode.com/2025/day/{day}/input");
            let content = client.get(&url).header("Cookie", format!("session={}", session)).send();
            if let Ok(Ok(text)) = content.map(|x| x.text()) {
                fs::write(&path, text.clone()).unwrap_or_else(|e| err!("Error writing file at {path}\n{e}"));
                text
            } else {
                err!("Error downloading file at {url}");
            }
        } else {
            err!("Could not find file at {path}. Did you mean to run with --download?");
        };

        let now = Instant::now();
        let (part_one, part_two) = problem.solve(&input);
        let elapsed = now.elapsed();

        println!("Day {day}");
        println!("--------");
        println!("Part one: {}", part_one.to_string());
        println!("Part two: {}", part_two.to_string());
        println!("Time elapsed: {}ms", elapsed.as_millis());
    });
}
