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

    let mut thickest_elf = Elf::default();

    let mut cur_cal_count = 0;
    let mut cur_pos = 1;
    for line in input_vec {
        if line == "" {
            if cur_cal_count > thickest_elf.total_cals {
                thickest_elf.pos = cur_pos;
                thickest_elf.total_cals = cur_cal_count;
            }

            cur_pos += 1;

            cur_cal_count = 0;
        } else {
            cur_cal_count += line.parse::<u32>().unwrap();
        }
    }

    println!(
        "The biggest elf is elf: {}, and he is carrying: {} calories worth of snacks",
        thickest_elf.pos, thickest_elf.total_cals
    );
}
