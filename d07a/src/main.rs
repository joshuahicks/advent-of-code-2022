use std::collections::HashMap;

const SIZE_GOAL: usize = 100000;

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

    let mut sum: usize = 0;
    for (_, v) in size_map {
        if v <= SIZE_GOAL {
            sum += v;
        }
    }

    println!("the total size for all directories that have a size of at most 100000 is: {sum}");
}
