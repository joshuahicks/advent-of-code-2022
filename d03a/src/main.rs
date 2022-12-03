use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();
    let input_vec = input_str.split("\n");

    let mut sum: u32 = 0;
    for line in input_vec {
        let (comp1, comp2) = line.split_at(line.len() / 2);
        
        for c in comp1.chars() {
            if comp2.contains(c) {
                sum += get_score(c);
                break;
            }
        }
    }
    println!{"The sum of priorities is: {}", sum};
}

fn get_score(c: char) -> u32 {
    let score;
    match c {
        'a'..='z' => score = c as u32 - 96,
        'A'..='Z' => score = c as u32 - 38,
        _ => panic!("Unrecognized character in input"),
    };
    score
}
