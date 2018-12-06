
pub fn day_five(input: Vec<String>) -> (String, String) {
    (part_one(&input), part_two(&input))
}

fn part_one(input: &Vec<String>) -> String {
    let mut char_vector: Vec<char> = input[0].chars().collect();
    let mut i: usize = 0;

    while i < char_vector.len() - 1 {
        let character = char_vector[i];
        let ascii_code = character as u8;
        
        if ascii_code + 32 == char_vector[i + 1] as u8 ||
           ascii_code - 32 == char_vector[i + 1] as u8 {
            char_vector.remove(i);
            char_vector.remove(i);
            if i >= 2 {
                i -= 2;
            }
            println!("{}{}", char_vector[i], char_vector[i + 1]);
        }
        i += 1;
    }

    char_vector.len().to_string()
}

fn part_two(input: &Vec<String>) -> String {
    String::new()
}
