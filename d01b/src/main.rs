use std::fs;

struct Elf {
    pos: u16,
    total_cals: u32,
}

impl Default for Elf {
    fn default() -> Self {
        Self { pos: 0, total_cals: 0}
    }
}

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();
    let input_vec = input_str.split("\n");

    let mut elf_vec = Vec::<Elf>::new();

    let mut cur_cal_count = 0;
    let mut cur_pos = 1;
    for line in input_vec {
        if line == "" {
            elf_vec.push(Elf {
                pos: cur_pos,
                total_cals: cur_cal_count,
            });

            cur_pos += 1;
            cur_cal_count = 0;
        } else {
            cur_cal_count += line.parse::<u32>().unwrap();
        }
    }

    //sort the vec in desc order
    elf_vec.sort_by(|a, b| b.total_cals.cmp(&a.total_cals));

    println!("The top 3 elves are:");
    println!("  Elf: {}, total_cals: {}", elf_vec[0].pos, elf_vec[0].total_cals);
    println!("  Elf: {}, total_cals: {}", elf_vec[1].pos, elf_vec[1].total_cals);
    println!("  Elf: {}, total_cals: {}", elf_vec[2].pos, elf_vec[2].total_cals);

    let total_sum = elf_vec[0].total_cals + elf_vec[1].total_cals + elf_vec[2].total_cals;
    println!("The sum of their calories is: {}", total_sum);
}
