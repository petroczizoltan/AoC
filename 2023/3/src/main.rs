use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();

    let mut sum1: i32 = 0;

    let file_lines = read_to_string(file_path).unwrap();
    let lines = file_lines.lines().into_iter().map(|x| x.into()).collect::<Vec<String>>();

    let mut number: i32 = 0;
    let mut is_reading_number = false;
    let mut good_number = false;

    for (line_index, line) in lines.iter().enumerate() {
        let last_line = match line_index {
            0 => None,
            _ => lines.get(line_index - 1)
        };
        let next_line = lines.get(line_index + 1);


        for (char_index, char) in line.chars().enumerate() {
            if is_reading_number && !char.is_digit(10) {
                if good_number {
                    sum1 += number;
                }
                number = 0;
                is_reading_number = false;
                good_number = false;
            }

            if char.is_digit(10) {
                is_reading_number = true;
                number *= 10;
                number += char.to_string().parse::<i32>().unwrap();
            }

            if is_reading_number && !good_number {
                good_number = check_if_has_symbol_around(line, &last_line, &next_line, char_index);
            }
        }
    }

    println!("1: {}", sum1);
}

fn check_if_has_symbol_around(line: &String, last_line: &Option<&String>, next_line: &Option<&String>, char_index: usize) -> bool {
    if let Some(line_chars) = last_line {
        if char_index > 0 {
            if let Some(left) = line_chars.chars().nth(char_index - 1) {
                if left != '.' && !left.is_digit(10) {
                    return true;
                }
            }
        }
        if let Some(center) = line_chars.chars().nth(char_index) {
            if center != '.' && !center.is_digit(10) {
                return true;
            }
        }
        if char_index < line.len() - 1 {
            if let Some(right) = line_chars.chars().nth(char_index + 1) {
                if right != '.' && !right.is_digit(10) {
                    return true;
                }
            }
        }
    }

    if char_index > 0 {
        if let Some(left) = line.chars().nth(char_index - 1) {
            if left != '.' && !left.is_digit(10) {
                return true;
            }
        }
    }
    if char_index < line.len() - 1 {
        if let Some(right) = line.chars().nth(char_index + 1) {
            if right != '.' && !right.is_digit(10) {
                return true;
            }
        }
    }

    if let Some(line_chars) = next_line {
        if char_index > 0{
            if let Some(left) = line_chars.chars().nth(char_index - 1) {
                if left != '.' && !left.is_digit(10) {
                    return true;
                }
            }
        }
        if let Some(center) = line_chars.chars().nth(char_index) {
            if center != '.' && !center.is_digit(10) {
                return true;
            }
        }
        if char_index < line.len() - 1 {
            if let Some(right) = line_chars.chars().nth(char_index + 1) {
                if right != '.' && !right.is_digit(10) {
                    return true;
                }
            }
        }
    }

    return false;
}
