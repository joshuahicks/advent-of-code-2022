use std::fs;

struct ElfAssignment {
    first_section: u32,
    last_section: u32,
}

impl ElfAssignment {
    fn make_new(sections: &str) -> Self {
        let f: Vec<&str> = sections.split("-").collect();

        Self {
            first_section: f[0].parse::<u32>().unwrap(),
            last_section: f[1].parse::<u32>().unwrap(),
        }
    }
}

fn main() {
    let input_str = fs::read_to_string("input.in").unwrap();
    let input_vec: Vec<&str> = input_str.split("\n").collect();

    let mut ans_count = 0;
    for line in input_vec {
        let elf_split: Vec<&str> = line.split(",").collect();

        if elf_split.len() != 2 {
            panic!("Error parsing input");
        }

        let elf_assign1 = ElfAssignment::make_new(elf_split[0]);
        let elf_assign2 = ElfAssignment::make_new(elf_split[1]);

        if inc_count(elf_assign1, elf_assign2) {
            ans_count += 1;
        }
    }

    println!(
        "There are {} pairs where one range fully contains the other",
        ans_count
    );
}

fn inc_count(e1: ElfAssignment, e2: ElfAssignment) -> bool {
    if e1.first_section <= e2.first_section && e1.last_section >= e2.first_section {
        return true;
    }
    if e2.first_section <= e1.first_section && e2.last_section >= e1.first_section {
        return true;
    }
    false
}
