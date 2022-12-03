use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let file_path = String::from("../input.txt");
    let data = convert_txt_to_vec(file_path);

    let test: i32 = data.iter().map(|s| get_round_score_pt2(s.to_string()) as i32).sum();
    println!("{:?}", test);
}

fn get_round_score(round: String) -> u8 {
    let player_1 = round.as_bytes()[0];
    let player_2 = round.as_bytes()[2] - 23;

    match (player_1, player_2) {
        (65, 67) | (66, 65) | (67, 66) => player_2 - 64,
        (p1, p2) if p1 == p2 => (player_2 - 64) + 3,
        _ => (player_2 - 64) + 6,
    }
}

fn get_round_score_pt2(round: String) -> u8 {
    let player_1 = round.as_bytes()[0];
    let player_2 = round.as_bytes()[2] - 23;

    match (player_1, player_2) {
        (66, 65) => 1,
        (67, 65) => 2,
        (65, 65) => 3,
        (67, 67) => 7,
        (65, 67) => 8,
        (66, 67) => 9,
        (_, 66) => (player_1 - 64) + 3,
        _ => 0,
    }
}

fn convert_txt_to_vec(txt_file_path: String) -> Vec<String> {
    let file = fs::File::open(txt_file_path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.expect("error")).collect()
}
