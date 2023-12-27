use std::{env, fs::read_to_string, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();
    let read_file = read_to_string(file_path).unwrap();
    let lines = read_file.lines().collect::<Vec<&str>>();

    let instructions = lines.first().unwrap();

    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;

    let mut location = "AAA";
    let mut locations= HashMap::new();

    for line in lines.iter().skip(2) {
        let [loc, _, l, r] = line.split(' ').collect::<Vec<&str>>()[..] else { panic!() };

        let left = l.get(1..=3).unwrap();
        let right = r.get(0..=2).unwrap();

        locations.insert(loc, (left, right));
    }

    let instructions_iter = instructions.chars().cycle();

    for instruction in instructions_iter {
        sum1 += 1;

        match instruction {
            'L' => location = &locations.get(location).unwrap().0,
            'R' => location = &locations.get(location).unwrap().1,
            _ => panic!(),
        }

        if location == "ZZZ" {
            break;
        }
    }

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}
