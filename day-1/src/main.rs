use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file_path = String::from("../input.txt");
    let cals_per_elf = convert_txt_to_vec(file_path);

    println!("Max: {}", part_one(&cals_per_elf));
    println!("Max Three Sum: {}", part_two(cals_per_elf));
}

fn part_one(cals_per_elf: &Vec<i32>) -> &i32 {
    cals_per_elf.iter().max().unwrap()
}

fn part_two(mut cals_per_elf: Vec<i32>) -> i32 {
    cals_per_elf.sort();
    cals_per_elf
        .split_off(cals_per_elf.len() - 3)
        .iter()
        .sum::<i32>()
}

fn convert_txt_to_vec(txt_file_path: String) -> Vec<i32> {
    let file = fs::File::open(txt_file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    let mut sums_per_whitespace_block = Vec::<i32>::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            sums_per_whitespace_block.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    sums_per_whitespace_block
}
