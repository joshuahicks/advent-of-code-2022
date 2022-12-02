use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();
    let input_vec = input_str.split("\n");

    let mut total_score = 0;
    for line in input_vec {
        let their_val = line.chars().next().unwrap(); // grab first value
        let my_val = line.chars().last().unwrap();    // grab last value

        match my_val {
            // ROCK
            'X' => match their_val {
                'A' => total_score += 1 + 3, // ROCK
                'B' => total_score += 1 + 0, // PAPER
                'C' => total_score += 1 + 6, // SCISSORS
                _ => panic!("Unrecognized input received"),
            },
            // PAPER
            'Y' => match their_val {
                'A' => total_score += 2 + 6, // ROCK
                'B' => total_score += 2 + 3, // PAPER
                'C' => total_score += 2 + 0, // SCISSORS
                _ => panic!("Unrecognized input received"),
            },
            // SCISSORS
            'Z' => match their_val {
                'A' => total_score += 3 + 0, // ROCK
                'B' => total_score += 3 + 6, // PAPER
                'C' => total_score += 3 + 3, // SCISSORS
                _ => panic!("Unrecognized input received"),
            },
            _ => panic!("Unrecognized input received"),
        }
    }

    println!("total score: {}", total_score);
}
