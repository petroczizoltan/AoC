use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();
    let read_file = read_to_string(file_path).unwrap();
    let lines = read_file.lines().collect::<Vec<&str>>();

    let times: Vec<i64> = lines.first().unwrap().split(' ').filter(|s| !s.is_empty()).skip(1).map(|s| s.parse::<i64>().unwrap()).collect();
    let distances: Vec<i64> = lines.last().unwrap().split(' ').filter(|s| !s.is_empty()).skip(1).map(|s| s.parse::<i64>().unwrap()).collect();

    let mut sum2: i64 = 0;

    let sum1 = times.iter().zip(distances.iter())
        .map(|(&time, &distance)| {
            let mut loading_time: i64 = 0;

            for load_time in 1..time {
                if (time - load_time) * load_time > distance {
                    loading_time = load_time;
                    break;
                }
            }

            return time - loading_time * 2 + 1;
        })
        .reduce(|acc, e| acc * e).unwrap();

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}
