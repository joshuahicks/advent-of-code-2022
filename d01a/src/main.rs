use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();
    let input_vec = input_str.split("\n");

    let mut most_cals = 0;
    let mut cur_cals = 0;

    for line in input_vec {
        if line == "" {
            if cur_cals > most_cals {
                most_cals = cur_cals;
            }

            cur_cals = 0;
        } else {
            cur_cals += line.parse::<u32>().unwrap();
        }
    }

    println!("The elf with most calories has {} calories", most_cals);
}
