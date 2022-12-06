mod instructions;

use crate::instructions::Instructions;
use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();
    let (crate_str, ins_str) = input_str.split_once("\n\n").unwrap();

    // setup crates vector
    let mut crates_vec: Vec<&str> = crate_str.split("\n").collect();
    let num_crates = crates_vec.last().unwrap().split_ascii_whitespace().count();
    crates_vec.pop(); // don't care about last row
    let mut crate_stack: Vec<Vec<char>> = vec![vec![]; num_crates];

    // build crate stack
    for line in crates_vec.into_iter().rev() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                crate_stack[i].push(c);
            }
        }
    }

    // setup and execute instructions
    let ins_vec: Vec<&str> = ins_str.split("\n").collect();
    let instructions: Vec<Instructions> = Instructions::make_new_vec(ins_vec);

    for ins in instructions {
        let mut temp_stack: Vec<char> = vec![]; 
        for _ in 0..ins.num_crates {
            let ch = crate_stack[ins.from_col].pop().unwrap();
            temp_stack.push(ch);   
        }

        for _ in 0..ins.num_crates {
            crate_stack[ins.to_col].push(temp_stack.pop().unwrap());
        }
    }

    print!("top of each stack: ");
    for c in &crate_stack[..] {
        print!("{}", c.last().unwrap());
    }
}
