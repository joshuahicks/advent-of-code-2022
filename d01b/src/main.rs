use std::fs;

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();
    let input_vec = input_str.split("\n");

    let mut cals_vec = Vec::<u32>::new();
    let mut cur_cals = 0;

    for line in input_vec {
        if line == "" {
            cals_vec.push(cur_cals);
            cur_cals = 0;
        } else {
            cur_cals += line.parse::<u32>().unwrap();
        }
    }

    //sort the vec in desc order
    cals_vec.sort_by(|a, b| b.cmp(a));

    let total_sum = cals_vec[0] + cals_vec[1] + cals_vec[2];
    println!(
        "The sum of calories the top three elves are carrying is: {}",
        total_sum
    );
}
