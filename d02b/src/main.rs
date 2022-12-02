use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();
    let input_vec = input_str.split("\n");

    let mut total_score = 0;
    for line in input_vec {
        let their_val = line.chars().next().unwrap(); // grab first value
        let outcome = line.chars().last().unwrap();   // grab last value

        match outcome {
            // LOSE
            'X' => match their_val {
                'A' => total_score += 0 + 3, // SCISSORS
                'B' => total_score += 0 + 1, // ROCK
                'C' => total_score += 0 + 2, // PAPER
                _ => panic!("Unrecognized input received"),
            },
            // TIE
            'Y' => match their_val {
                'A' => total_score += 3 + 1, // ROCK
                'B' => total_score += 3 + 2, // PAPER
                'C' => total_score += 3 + 3, // SCISSORS
                _ => panic!("Unrecognized input received"),
            },
            // WIN
            'Z' => match their_val {
                'A' => total_score += 6 + 2, // PAPER
                'B' => total_score += 6 + 3, // SCISSORS
                'C' => total_score += 6 + 1, // ROCK
                _ => panic!("Unrecognized input received"),
            },
            _ => panic!("Unrecognized input received"),
        }
    }

    println!("total score: {}", total_score);
}
