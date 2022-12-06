use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();

    let mut hist_queue: Vec<char> = vec![];
    for (i, c) in input_str.chars().enumerate() {
        if hist_queue.len() == 4 {
            if check_for_marker(&hist_queue) {
                println!("{} characters need to be processed", i);
                break;
            } 
            hist_queue.remove(0);
        }
        hist_queue.push(c);
    }
}

fn check_for_marker(q: &Vec<char>) -> bool {
    for c in q {
        if q.iter().filter(|&n| *n == *c).count() > 1 {
            return false
        }
    }
    true
}