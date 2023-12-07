use std::{env, fs::read_to_string, collections::HashSet};

#[derive(Debug)]
pub struct PossibleGear {
    gears: (i32, Option<i32>),
    coords: (usize, usize),
    resolved: bool
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;

    let file_lines = read_to_string(file_path).unwrap();
    let lines = file_lines.lines().into_iter().map(|x| x.into()).collect::<Vec<String>>();

    let mut number: i32 = 0;
    let mut is_reading_number = false;
    let mut good_number = false;
    let mut possible_gears: Vec<PossibleGear> = Vec::new();
    let mut possible_coords: HashSet<(usize, usize)> = HashSet::new();

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
                for coords in possible_coords.iter() {
                    let mut found = false;
                    for possible_gear in possible_gears.iter_mut() {
                        if possible_gear.coords == *coords && !possible_gear.resolved {
                            found = true;
                            possible_gear.gears.1 = Some(number);
                            possible_gear.resolved = true;
                        }
                    }
                    if !found {
                        possible_gears.push(PossibleGear {
                            gears: (number, None),
                            coords: *coords,
                            resolved: false,
                        });
                    }
                }
                number = 0;
                is_reading_number = false;
                good_number = false;
                possible_coords.clear();
            }

            if char.is_digit(10) {
                is_reading_number = true;
                number *= 10;
                number += char.to_string().parse::<i32>().unwrap();
            }

            if is_reading_number {
                let (_good_number, coords) = check_if_has_symbol_around(line, &last_line, &next_line, line_index, char_index);
                if !good_number {
                    good_number = _good_number
                }
                if let Some(coord) = coords {
                    possible_coords.insert(coord);
                }
            }
        }
    }

    dbg!(&possible_gears);
    for possible_gear in possible_gears {
        if possible_gear.resolved {
            let (a, b) = possible_gear.gears;
            sum2 += a * b.unwrap();
        }
    }

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}

fn check_if_has_symbol_around(line: &String, last_line: &Option<&String>, next_line: &Option<&String>, line_index: usize, char_index: usize) -> (bool, Option<(usize, usize)>) {
    if let Some(line_chars) = last_line {
        if char_index > 0 {
            if let Some(left) = line_chars.chars().nth(char_index - 1) {
                if left != '.' && !left.is_digit(10) {
                    let coord = if left == '*' { Some((line_index - 1, char_index - 1)) } else { None };
                    return (true, coord);
                }
            }
        }
        if let Some(center) = line_chars.chars().nth(char_index) {
            if center != '.' && !center.is_digit(10) {
                let coord = if center == '*' { Some((line_index - 1, char_index)) } else { None };
                return (true, coord);
            }
        }
        if char_index < line.len() - 1 {
            if let Some(right) = line_chars.chars().nth(char_index + 1) {
                if right != '.' && !right.is_digit(10) {
                    let coord = if right == '*' { Some((line_index - 1, char_index + 1)) } else { None };
                    return (true, coord);
                }
            }
        }
    }

    if char_index > 0 {
        if let Some(left) = line.chars().nth(char_index - 1) {
            if left != '.' && !left.is_digit(10) {
                let coord = if left == '*' { Some((line_index, char_index - 1)) } else { None };
                return (true, coord);
            }
        }
    }
    if char_index < line.len() - 1 {
        if let Some(right) = line.chars().nth(char_index + 1) {
            if right != '.' && !right.is_digit(10) {
                let coord = if right == '*' { Some((line_index, char_index + 1)) } else { None };
                return (true, coord);
            }
        }
    }

    if let Some(line_chars) = next_line {
        if char_index > 0{
            if let Some(left) = line_chars.chars().nth(char_index - 1) {
                if left != '.' && !left.is_digit(10) {
                    let coord = if left == '*' { Some((line_index + 1, char_index - 1)) } else { None };
                    return (true, coord);
                }
            }
        }
        if let Some(center) = line_chars.chars().nth(char_index) {
            if center != '.' && !center.is_digit(10) {
                let coord = if center == '*' { Some((line_index + 1, char_index)) } else { None };
                return (true, coord);
            }
        }
        if char_index < line.len() - 1 {
            if let Some(right) = line_chars.chars().nth(char_index + 1) {
                if right != '.' && !right.is_digit(10) {
                    let coord = if right == '*' { Some((line_index + 1, char_index + 1)) } else { None };
                    return (true, coord);
                }
            }
        }
    }

    return (false, None);
}
