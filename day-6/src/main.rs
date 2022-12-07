use std::collections::HashSet;
use std::io::Read;
use std::{fs, io};

fn main() {
    let file_path = String::from("../input.txt");
    let data = convert_txt_to_string(file_path);
    let marker_start = get_unique_window_index(data);
    println!("Marker start: {}", marker_start);
}

fn get_unique_window_index(passcode: Vec<char>) -> usize {
    // Part 1 = 4, Part 2 = 14
    const WINDOW_LENGTH: usize = 14;

    for (i, win) in passcode.windows(WINDOW_LENGTH).enumerate() {
        let mut set: HashSet<usize> = HashSet::new();

        for (j, ch) in win.iter().enumerate() {
            if !set.insert(*ch as usize) {
                set.clear();
                break;
            } else if j == WINDOW_LENGTH - 1 {
                return i + WINDOW_LENGTH;
            }
        }
    }
    0
}

fn convert_txt_to_string(path: String) -> Vec<char> {
    let file = fs::File::open(path).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut passcode = String::new();

    reader.read_to_string(&mut passcode).unwrap();
    passcode.pop();
    passcode.chars().collect::<Vec<char>>()
}
