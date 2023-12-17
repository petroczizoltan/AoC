use core::panic;
use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let [winning_numbers, your_numbers] = &line.split(": ").collect::<Vec<&str>>()
            .last().unwrap().split(" | ")
            .map(|s| s.split(" ")
                 .filter(|s| s.to_string() != "")
                 .map(|s| s.parse::<i32>().unwrap())
                 .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()[..] else { panic!() };

        let mut your_winning_number_count = 0;

        for your_number in your_numbers {
            if winning_numbers.contains(your_number) {
                your_winning_number_count += 1;
            }
        }

        if your_winning_number_count == 0 {
            continue;
        } else {
            sum1 += 0b1 << (your_winning_number_count - 1);
        }
    }

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}
