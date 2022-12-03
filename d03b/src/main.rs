use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();
    let input_vec: Vec<&str> = input_str.split("\n").collect();

    let mut sum: u32 = 0;
    for i in (0..input_vec.len()).step_by(3) { 
        let elf1 = input_vec[i];
        let elf2 = input_vec[i+1];
        let elf3 = input_vec[i+2];
        
        for c in elf1.chars() {
            if elf2.contains(c) && elf3.contains(c) {
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