use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file_path = String::from("../input.txt");
    let cals_per_elf = convert_txt_to_vec(file_path);

    println!("Max: {}", part_one(&cals_per_elf));
    println!("Max Three Sum: {} - O", part_two_optimal(&cals_per_elf));
    println!("Max Three Sum: {}", part_two(cals_per_elf));
}

fn part_one(cals_per_elf: &Vec<i32>) -> &i32 {
    cals_per_elf.iter().max().unwrap()
}

// 4.1036 microseconds
fn part_two(mut cals_per_elf: Vec<i32>) -> i32 {
    cals_per_elf.sort();
    cals_per_elf
        .split_off(cals_per_elf.len() - 3)
        .iter()
        .sum::<i32>()
}

// 284.61 nanoseconds
fn part_two_optimal(cals_per_elf: &Vec<i32>) -> i32 {
    let mut max_one = 0;
    let mut max_two = 0;
    let mut max_three = 0;

    for num in cals_per_elf.iter() {
        if *num > max_one {
            max_two = max_one;
            max_one = *num;
        } else if *num > max_two {
            max_three = max_two;
            max_two = *num;
        } else if *num > max_three {
            max_three = *num;
        }
    }
    max_one + max_two + max_three
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
