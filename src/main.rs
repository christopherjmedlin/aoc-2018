use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

extern crate regex;
extern crate chrono;

mod dayone;
mod daytwo;
mod daythree;
mod dayfour;
mod dayfive;
mod daysix;
mod dayseven;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("No day specified.");
    } else {
        let (part_one, part_two) = run_day(args[1].as_str());
        println!("Part 1: {}", part_one);
        println!("Part 2: {}", part_two);
    }
}

fn run_day(day: &str) -> (String, String) {
    let f = File::open(format!("inputs/day{}.txt", day)).expect("no such file");
    let r = BufReader::new(f);
    let mut input: Vec<String> = r.lines()
                                  .map(|r| r.expect("Could not parse line"))
                                  .collect();


    match day {
        "1" => {dayone::day_one(input)},
        "2" => {daytwo::day_two(input)},
        "3" => {daythree::day_three(input)},
        "4" => {dayfour::day_four(input)},
        "5" => {dayfive::day_five(input)},
        "6" => {daysix::day_six(input)},
        "7" => {dayseven::day_seven(input)},
        &_ => {(String::from("unimplemented"), String::from("unimplemented"))}
    }
}
