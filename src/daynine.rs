
pub fn day_nine(input: Vec<String>) -> (String, String) {
    let split: Vec<&str> = input[0].split(" ").collect();
    let players: u32 = split[0].parse().unwrap();
    let marbles: u32 = split[6].parse().unwrap();
    (part_one(players, marbles), part_two(players, marbles))
}

fn part_one(players: u32, marbles: u32) -> String {
    let mut played_marbles: Vec<u32> = vec!(0);
    let mut current_marble_index: usize = 0;
    let mut current_player: usize = 0;
    let mut player_scores: Vec<u32> = Vec::with_capacity(players as usize);
    for i in 0..players {
        player_scores.push(0);
    }

    for marble in 1..marbles + 1 {
        if marble % 23 == 0 {
            player_scores[current_player] += marble;
            let signed_index = (current_marble_index as i32) - 7;
            let signed_length = played_marbles.len() as i32;
            current_marble_index = ((signed_index % signed_length + signed_length) % signed_length) as usize;
            player_scores[current_player] += played_marbles[current_marble_index];
            played_marbles.remove(current_marble_index);

        } else {
            current_marble_index = ((current_marble_index + 1) 
                                    % played_marbles.len()) + 1;
            played_marbles.insert(current_marble_index, marble); 
        }

            println!("{}", played_marbles[current_marble_index]);
        current_player += 1;
        current_player %= player_scores.len();
    }

    let mut high_score: u32 = 0;
    for score in player_scores.iter() {
        if *score > high_score {
            high_score = *score;
        }
    }

    high_score.to_string()   
}

fn part_two(players: u32, marbles: u32) -> String {
    String::from("¯\\_(ツ)_/¯")
}
