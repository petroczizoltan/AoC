use core::panic;
use regex::Regex;
use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();

    let re = Regex::new(r"[0-9]").unwrap();
    let re2 = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let rre2 = Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let rline = line.chars().rev().collect::<String>();

        let numbers1: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let numbers2: Vec<&str> = re2.find_iter(line).map(|m| m.as_str()).collect();

        let rnumbers2: Vec<&str> = rre2.find_iter(&rline).map(|m| m.as_str()).collect();
        let first1 = numbers1.first();
        let last1 = numbers1.last();
        let first2 = numbers2.first();
        let last2 = rnumbers2.first();
        let first_number1 = try_parse_string_to_i32(&first1, None);
        let first_number2 = try_parse_string_to_i32(&first2, None);
        let last_number1 = try_parse_string_to_i32(&last1, Some(first_number1));
        let last_number2 = try_parse_string_to_i32(&last2, Some(first_number2));
        sum1 += first_number1 * 10 + last_number1;
        sum2 += first_number2 * 10 + last_number2;
    }

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}

fn try_parse_string_to_i32(str: &Option<&&str>, default: Option<i32>) -> i32 {
        return match &str {
            Some(&"one") => 1,
            Some(&"eno") => 1,
            Some(&"two") => 2,
            Some(&"owt") => 2,
            Some(&"three") => 3,
            Some(&"eerht") => 3,
            Some(&"four") => 4,
            Some(&"ruof") => 4,
            Some(&"five") => 5,
            Some(&"evif") => 5,
            Some(&"six") => 6,
            Some(&"xis") => 6,
            Some(&"seven") => 7,
            Some(&"neves") => 7,
            Some(&"eight") => 8,
            Some(&"thgie") => 8,
            Some(&"nine") => 9,
            Some(&"enin") => 9,
            Some(num) => num.parse::<i32>().unwrap(),
            None => {
                match default {
                    Some(v) => v,
                    None => panic!(),
                }
            },
        };
}
