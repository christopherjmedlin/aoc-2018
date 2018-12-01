use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

mod dayone;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("No day specified.");
    } else {
        println!("{}", run_day(args[1].as_str()));
    }
}

fn run_day(day: &str) -> String {
    let f = File::open(format!("inputs/day{}.txt", day)).expect("no such file");
    let r = BufReader::new(f);
    let mut input: Vec<String> = r.lines()
                                  .map(|r| r.expect("Could not parse line"))
                                  .collect();


    match day {
        "1" => {dayone::day_one(input)}
        &_ => {format!("Day {} has not been implemented", day)}
    }
}
