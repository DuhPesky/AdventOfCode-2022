use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let file_path = String::from("../input2.txt");
    let data = convert_txt_to_vec(file_path);
    // let score: i32 = data.iter().map(|item| get_priority_sum(item) as i32).sum();
    let score_2: i32 = data
        .chunks(3)
        .map(|chunk| get_badge_sum(chunk) as i32)
        .sum();

    println!("Score: {}", score_2);
}

// Part one
fn get_priority_sum(items: &String) -> u8 {
    let split_point = items.as_bytes().len() / 2;
    let mut unique_items = HashSet::new();

    for (i, item) in items.as_bytes().iter().enumerate() {
        if i < split_point {
            unique_items.insert(item);
        } else if unique_items.contains(&item) {
            return if *item >= 97 { *item - 96 } else { *item - 38 };
        }
    }
    0
}

// Part Two
fn get_badge_sum(items: &[String]) -> u8 {
    let sets: Vec<HashSet<char>> = items.iter().map(|s| s.chars().collect()).collect();

    let three_set_intersection = sets[0]
        .intersection(&sets[1])
        .find(|ch| sets[2].contains(ch));

    if let Some(ch) = three_set_intersection {
        let ch_value = *ch as u8;
        return if ch_value >= 97 {
            ch_value - 96
        } else {
            ch_value - 38
        };
    }
    0
}

fn convert_txt_to_vec(txt_file_path: String) -> Vec<String> {
    let file = fs::File::open(txt_file_path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.expect("error")).collect()
}
