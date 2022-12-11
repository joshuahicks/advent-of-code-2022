use std::collections::HashMap;

const AVAILABLE_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

fn main() {
    let input = include_str!("../input.in");    

    let mut size_map: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut pwd_stack: Vec<&str> = Vec::new();

    for line in input.lines() {
        let contents: Vec<&str> = line.split_ascii_whitespace().collect();

        match contents[0] {
            "$" => {
                if contents[1] == "cd" {
                    match contents[2] {
                        ".." => {
                            pwd_stack.pop();
                        }
                        _ => {
                            pwd_stack.push(contents[2]);
                            size_map.insert(pwd_stack.clone(), 0);
                        }
                    }
                }
            }
            "dir" => {}
            _ => {
                let size = contents[0].parse::<usize>().unwrap();

                for i in (1..pwd_stack.len()+1).rev() {
                    *size_map.get_mut(&pwd_stack[0..i]).unwrap() += size;
                }
            }
        }
    }

    let free_space = AVAILABLE_SIZE - size_map.get(&vec!["/"]).unwrap();
    let space_needed = UPDATE_SIZE - free_space;

    let smallest_delete = *size_map.values().filter(|&val| val >= &space_needed).min().unwrap();

    println!("the size of the smallest folder that can be deleted to make enough room for the update is: {smallest_delete}");
}
