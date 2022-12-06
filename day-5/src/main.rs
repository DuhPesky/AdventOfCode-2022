use std::io::BufRead;
use std::{fs, io};

fn main() {
    let input_path = String::from("../input.txt");
    let data = convert_txt_to_vec(input_path);

    let mut bins = make_bin_vec(&data);
    let crane_operations = make_operations_vec(&data);

    bins = bins
        .into_iter()
        .map(|row| row.into_iter().rev().collect())
        .collect();

    bins = move_items(bins, crane_operations);
    for stack in &bins {
        print!("{}", char::from_u32(*stack.last().unwrap() as u32).unwrap());
    }
}

// Part 1 and 2
fn move_items(mut bins: Vec<Vec<usize>>, operations: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    for operation in operations {
        let amount = operation[0];
        let from_bin = operation[1] - 1;
        let to_bin = operation[2] - 1;

        let final_length = bins[from_bin].len().saturating_sub(amount);

        let cutoff: Vec<usize> = bins[from_bin].split_off(final_length);
        // Part 1 (uncomment)
        // cutoff.reverse();

        bins[to_bin].extend_from_slice(&cutoff);
    }
    bins
}

fn make_bin_vec(data: &Vec<String>) -> Vec<Vec<usize>> {
    let mut bins: Vec<Vec<usize>> = Vec::new();

    for (i, line) in data.iter().enumerate() {
        for (j, four_ch) in line.as_bytes().chunks(4).enumerate() {
            if i == 0 {
                bins.push(vec![]);
            }
            if four_ch[0] != 91 {
                continue;
            }
            bins[j].push(four_ch[1] as usize);
        }
    }
    bins
}

fn make_operations_vec(data: &Vec<String>) -> Vec<Vec<usize>> {
    let mut crane_operations: Vec<Vec<usize>> = vec![];

    for line in data.iter() {
        if !line.contains("move") {
            continue;
        }

        let instruction: Vec<&str> = line.split_whitespace().collect();
        let amount = instruction[1].parse::<usize>().unwrap();
        let from_bin = instruction[3].parse::<usize>().unwrap();
        let to_bin = instruction[5].parse::<usize>().unwrap();

        crane_operations.push(vec![amount, from_bin, to_bin]);
    }
    crane_operations
}

fn convert_txt_to_vec(file_path: String) -> Vec<String> {
    let file = fs::File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}
