use core::panic;
use std::{env, fs::read_to_string, iter::Enumerate, io::Lines};

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();
    let read_file = read_to_string(file_path).unwrap();
    let lines = read_file.lines().collect::<Vec<&str>>();

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;

    let mut won_copies: Vec<i32> = vec![1; lines.len()];

    for (index, line) in lines.iter().enumerate() {
        let [winning_numbers, your_numbers] = &line.split(": ").collect::<Vec<&str>>()
            .last().unwrap().split(" | ")
            .map(|s| s.split(" ")
                 .filter(|s| s.to_string() != "")
                 .map(|s| s.parse::<i32>().unwrap())
                 .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()[..] else { panic!() };

        let mut your_winning_number_count: i32 = 0;

        for your_number in your_numbers {
            if winning_numbers.contains(your_number) {
                your_winning_number_count += 1;
            }
        }

        for i in 0..your_winning_number_count {
            let this_card_count = won_copies[index];
            won_copies[index + (i as usize) + 1] += this_card_count;
        }
        sum2 += won_copies[index];

        if your_winning_number_count == 0 {
            continue;
        } else {
            sum1 += 0b1 << (your_winning_number_count - 1);
        }
    }

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}
