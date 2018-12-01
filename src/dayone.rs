
pub fn day_one(input: Vec<String>) -> String {
    let mut frequency: i32 = 0;
    let mut reached_frequencies: Vec<i32> = Vec::new();
    let mut result_found: bool = false;
    
    while !result_found {
        for line in input.iter() {
            let sign = &line[0..1];
            let num: &i32 = &line[1..line.len()]
                      .parse::<i32>().unwrap();

            if sign == "+" {
                frequency += num;    
            } else {
                frequency -= num;
            }
            
            println!("{}", reached_frequencies.len());
            if reached_frequencies.contains(&frequency) {
                result_found = true;
                break;
            }
            reached_frequencies.push(frequency);
        }
    }

    frequency.to_string()
}
