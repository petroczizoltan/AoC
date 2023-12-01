use core::panic;
use regex::Regex;
use std::{env, fs::read_to_string, path::PathBuf, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    let relative_input_file_path = &args[1];

    let input_file_path = match env::current_dir() {
        Ok(path) => {
            let mut file_path = PathBuf::new();
            file_path.push(path);
            file_path.push(relative_input_file_path);
            let absolue_path = file_path.as_path().display().to_string();
            absolue_path
        },
        Err(_) => panic!(),
    };

    let re = Regex::new(r"[0-9]").unwrap();

    let mut sum: i32 = 0;

    for line in read_to_string(input_file_path).unwrap().lines() {
        let numbers: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let first_number = match &numbers.first() {
            Some(num) => num.parse::<i32>().unwrap(),
            None => panic!(),
        };
        let last_number = match &numbers.last() {
            Some(num) => num.parse::<i32>().unwrap(),
            None => first_number,
        };
        sum += first_number * 10 + last_number;
    }

    println!("{}", sum);
}
