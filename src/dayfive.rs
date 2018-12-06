
pub fn day_five(input: Vec<String>) -> (String, String) {
    (part_one(&input), part_two(&input))
}

fn part_one(input: &Vec<String>) -> String {
    let mut char_vector: Vec<char> = input[0].chars().collect();
    let mut i: usize = 0;

    while i < char_vector.len() - 1 {
        let character = char_vector[i];
        let ascii_code = character as u8;
        let mut reaction_found = false;
        
        if ascii_code + 32 == char_vector[i + 1] as u8 ||
           ascii_code - 32 == char_vector[i + 1] as u8 {
            char_vector.remove(i);
            char_vector.remove(i);
            if i >= 2 {
                i -= 2;
            }
            reaction_found = true;
        }
        if !reaction_found {
            i += 1;
        }
    }

    char_vector.len().to_string()
}

fn part_two(input: &Vec<String>) -> String {
    let char_vector: Vec<char> = input[0].chars().collect();
    
    let mut smallest_length: usize = 50000;
    for ascii in 65..=90 {
        let mut new_char_vector = char_vector.clone();
        let mut i: usize = 0;
        
        let mut i = 0;
        while i < new_char_vector.len() - 1 {
            let ascii_code = new_char_vector[i] as u8;

            if ascii_code == ascii || ascii_code == ascii + 32 {
                println!("{}", new_char_vector[i]);
                new_char_vector.remove(i);
            }
            
            i += 1;
        }
        
        let length = new_char_vector.len();
        if length < smallest_length {
            smallest_length = length;
        }
    }

    smallest_length.to_string()
}
