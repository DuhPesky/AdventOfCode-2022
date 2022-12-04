use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let file_path = String::from("../input.txt");
    let data = convert_txt_to_vec(file_path);
    // let score: i32 = data.iter().map(|pair| check_full_overlap(pair)).sum();
    let score: i32 = data.iter().map(|pair| check_any_overlap(pair)).sum();
    println!("Total overlapping: {}", score);
}

// Part One
fn check_full_overlap(data: &String) -> i32 {
    let data_split: Vec<&str> = data.split(|ch| ch == '-' || ch == ',').collect();

    let block_1 = data_split[0].parse::<i32>().unwrap();
    let block_2 = data_split[1].parse::<i32>().unwrap();
    let block_3 = data_split[2].parse::<i32>().unwrap();
    let block_4 = data_split[3].parse::<i32>().unwrap();

    if block_1 <= block_3 && block_2 >= block_4 {
        return 1;
    }
    if block_3 <= block_1 && block_4 >= block_2 {
        return 1;
    }
    0
}

// Part Two
fn check_any_overlap(data: &String) -> i32 {
    let data_split: Vec<&str> = data.split(|ch| ch == '-' || ch == ',').collect();

    let block_1 = data_split[0].parse::<i32>().unwrap();
    let block_2 = data_split[1].parse::<i32>().unwrap();
    let block_3 = data_split[2].parse::<i32>().unwrap();
    let block_4 = data_split[3].parse::<i32>().unwrap();

    if std::cmp::max(block_1, block_3) <= std::cmp::min(block_2, block_4) {
        return 1;
    }
    0
}

fn convert_txt_to_vec(file_path: String) -> Vec<String> {
    let file = fs::File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}
