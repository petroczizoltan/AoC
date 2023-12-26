use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();
    let read_file = read_to_string(file_path).unwrap();
    let lines = read_file.lines().collect::<Vec<&str>>();

    let times1: Vec<i64> = lines.first().unwrap().split(' ').filter(|s| !s.is_empty()).skip(1).map(|s| s.parse::<i64>().unwrap()).collect();
    let distances1: Vec<i64> = lines.last().unwrap().split(' ').filter(|s| !s.is_empty()).skip(1).map(|s| s.parse::<i64>().unwrap()).collect();

    let time2 = lines.first().unwrap().split(' ').filter(|s| !s.is_empty()).skip(1).collect::<Vec<&str>>().join("").parse::<i64>().unwrap();
    let distance2 = lines.last().unwrap().split(' ').filter(|s| !s.is_empty()).skip(1).collect::<Vec<&str>>().join("").parse::<i64>().unwrap();

    let sum1 = times1.iter().zip(distances1.iter())
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

    let range_vec = (1..time2).collect::<Vec<i64>>();
    let load_time2 = range_vec.iter()
        .skip_while(|&&load_time| {
            return (time2 - load_time) * load_time <= distance2;
        })
        .next().unwrap();
    let sum2 = time2 - load_time2 * 2 + 1;

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}
