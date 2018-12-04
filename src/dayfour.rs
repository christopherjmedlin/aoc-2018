use regex::Regex;
use chrono::prelude::*;

pub fn day_four(input: Vec<String>) -> (String, String) {
    let mut sorted_input = input.clone();
    sorted_input.sort();

    (part_one(&sorted_input), part_two(&sorted_input))
}

struct Record {
    time: DateTime<Utc>,

    record_type: String,
    guard_number: Option<u16>,
}

fn part_one(input: &Vec<String>) -> String {
    let re = Regex::new(r"\[\d{4}-(\d{2})-(\d{2}) (\d{2}):(\d{2})] ([A-z]*) ([#-z]*)")
                    .unwrap();
    let mut records: Vec<Record> = Vec::new();

    for line in input.iter() {
        let captures = re.captures(line).unwrap();
        
        let mut record_type = captures.get(5).unwrap().as_str();
        let mut guard_number: Option<u16> = None;
        if record_type == "Guard" {
            record_type = "guard";
            let guard_number_str = String::from(captures.get(6).unwrap().as_str());
            guard_number = Some(guard_number_str[1..].parse().unwrap())
        } else if record_type == "falls" {
            record_type = "sleep";
        } else {
            record_type = "wake";
        }
        
        let month = captures.get(1).unwrap().as_str().parse().unwrap();
        let day = captures.get(2).unwrap().as_str().parse().unwrap();
        let hour = captures.get(3).unwrap().as_str().parse().unwrap();
        let minute = captures.get(4).unwrap().as_str().parse().unwrap();

        let record = Record {
            time: Utc.ymd(1518, month, day).and_hms(hour, minute, 0),

            record_type: String::from(record_type),
            guard_number: guard_number
        };
        records.push(record);
    }

    let current_guard = 0;
    let is_sleeping = false;
    let sleep_time: DateTime<Utc>;
    let guard_map: HashMap<u16, u16>;

    for record in records.iter() {
        if record.record_type == "guard" {
            current_guard = record.guard_number;
            is_sleeping = false;
        } else if record.record_type == "sleep" {
            if !is_sleeping {
                sleep_time = record.time;
            }
            is_sleeping = true;
        } else {
            is_sleeping = false;
        }

        if (record.record_type == "wake" || record.record_type == "guard") && is_sleeping {
            
        }
    }
    String::new()
}

fn part_two(input: &Vec<String>) -> String {
    
    String::new()
}
