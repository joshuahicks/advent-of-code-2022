pub mod instructions {}

pub struct Instructions {
    pub num_crates: usize,
    pub from_col: usize,
    pub to_col: usize,
}

impl Instructions {
    pub fn make_new_vec(input: Vec<&str>) -> Vec<Self> {
        let mut instructions = Vec::<Instructions>::new();
        for line in input {
            let line_split: Vec<&str> = line.split(" ").collect();
            instructions.push(Instructions {
                num_crates: line_split[1].parse::<usize>().unwrap(),
                from_col: line_split[3].parse::<usize>().unwrap()-1,
                to_col: line_split[5].parse::<usize>().unwrap()-1,
            });
        }
        instructions
    }
}
