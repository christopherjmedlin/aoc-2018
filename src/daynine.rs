
pub fn day_nine(input: Vec<String>) -> (String, String) {
    let split: Vec<&str> = input[0].split(" ").collect();
    let players: u32 = split[0].parse().unwrap();
    let marbles: u32 = split[6].parse().unwrap();
    (highest_score(players, marbles), highest_score(players, marbles * 100))
}

struct CircularDoublyLinkedList {
    nodes: Vec<Node>,
    cursor: Option<usize>,
}

struct Node {
    val: u32,
    prev: usize,
    next: usize,
}

impl CircularDoublyLinkedList {
    pub fn new() -> CircularDoublyLinkedList {
        CircularDoublyLinkedList {
            nodes: Vec::new(),
            cursor: None
        }
    }
    
    // Inserts after current node
    pub fn insert(&mut self, val: u32) {
        let i = self.nodes.len();

        let (prev, next) = match self.cursor {
            Some(current) => {
                let next = self.nodes[current].next;
                self.nodes[next].prev = i;
                self.nodes[current].next = i;
                (current, next)
            },
            None => {
                (i, i)
            }
        };
        
        self.nodes.push(Node {
            val: val,
            prev: prev,
            next: next,
        });
        self.cursor = Some(i);
    }
    
    // Removes current node
    pub fn remove(&mut self) {
        let prev = self.nodes[self.cursor.unwrap()].prev;
        let next = self.nodes[self.cursor.unwrap()].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
        self.cursor = Some(next);
    }
    
    // Moves cursor back <num> nodes
    pub fn back(&mut self, num: usize) {
        if self.cursor != None {
            let mut cursor = self.cursor.unwrap();
            for i in 0..num {
                cursor = self.nodes[cursor].prev;  
            }

            self.cursor = Some(cursor);
        }
    }

    // Moves cursor forward <num> nodes
    pub fn forward(&mut self, num: usize) {
        if self.cursor != None {
            let mut cursor = self.cursor.unwrap();
            for i in 0..num {
                cursor = self.nodes[cursor].next;  
            }

            self.cursor = Some(cursor);
        }
    }

    // Gets current node value
    pub fn current(&mut self) -> Option<u32> {
        match self.cursor {
            Some(current) => {
                Some(self.nodes[current].val)
            }
            None => {
                None
            }
        }
    }
}

fn highest_score(players: u32, marbles: u32) -> String {
    let mut played_marbles = CircularDoublyLinkedList::new();
    let mut current_player: usize = 0;
    let mut player_scores: Vec<u32> = Vec::with_capacity(players as usize);
    for i in 0..players {
        player_scores.push(0);
    }
    played_marbles.insert(0);

    for marble in 1..marbles + 1 {
        if marble % 23 == 0 {
            player_scores[current_player] += marble;
            played_marbles.back(7);
            
            player_scores[current_player] += played_marbles.current().unwrap();
            played_marbles.remove();
        } else {
            played_marbles.forward(1);
            played_marbles.insert(marble); 
        }

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
