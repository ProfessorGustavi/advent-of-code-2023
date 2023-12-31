use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input.txt").expect("Error opening file");
    let vector = get_symbol_locations(file);
    let file = File::open("./input.txt").expect("Error opening file");
    let result = get_number_results(file,vector);
    println!("{}", result);
}


fn get_symbol_locations(file: File) -> Vec<(u32, u32)> {
    let mut symbol_indexes: Vec<(u32, u32)> = Vec::new();
    let reader = io::BufReader::new(file);

    for (line_idx, line_result) in reader.lines().enumerate() {
        if let Ok(line) = line_result {
            for (char_idx, ch) in line.chars().enumerate() {
                if !ch.is_digit(10) && ch != '.' {
                    symbol_indexes.push((char_idx as u32, line_idx as u32));
                }
            }
        }
    }
    symbol_indexes
}

fn get_number_results(file: File, char_positions: Vec<(u32, u32)>) -> u32 {
    let mut map: HashMap<(u32,u32), u32> = HashMap::new();
    let mut result = 0;
    let mut last = 0;
    let reader = io::BufReader::new(file);
    let mut length_of_line;
    for (line_idx, line_result) in reader.lines().enumerate() {
        if let Ok(line) = line_result {
            length_of_line = line.len() as u32;
            for (char_idx, ch) in line.chars().enumerate() {
                if ch.is_digit(10){
                    let curr_idx = (char_idx as u32, line_idx as u32);
                    for el in &char_positions {
                        if el.0.abs_diff(curr_idx.0) < 2 && el.1.abs_diff(curr_idx.1) < 2
                        {
                            let rs = find_whole_number(&line, char_idx);
                            if rs != last {
                                last = rs;
                                if map.contains_key(el) {
                                    result += rs * map.get(el).unwrap();
                                }
                                map.insert(*el,rs);
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn find_whole_number(line: &str, index: usize) -> u32 {
    let mut start_index = index;
    let mut end_index = index;

    while end_index < line.len() && line.chars().nth(end_index).unwrap().is_digit(10) {
        end_index += 1;
    }
    while start_index > 0 && line.chars().nth(start_index - 1).unwrap().is_digit(10) {
        start_index -= 1;
    }
    let whole_number: String = line[start_index..end_index].to_string();
    whole_number.parse().unwrap_or(0)
}

