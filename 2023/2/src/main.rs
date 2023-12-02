use regex::{Regex, Captures, Match};
use std::{env, fs::read_to_string};

pub trait MatchParse {
    fn to_i32(&self) -> i32;
    fn to_color(&self) -> CubeColor;
}

impl MatchParse for Option<Match<'_>> {
    fn to_i32(&self) -> i32 {
        return self.unwrap().as_str().parse::<i32>().unwrap();
    }
    fn to_color(&self) -> CubeColor {
        return match self.unwrap().as_str() {
            "red" => CubeColor::Red,
            "green" => CubeColor::Green,
            "blue" => CubeColor::Blue,
            _ => panic!(),
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();

    let game_re = Regex::new(r"Game ([0-9]+):").unwrap();
    let cube_re = Regex::new(r"([0-9]+) (red|blue|green)").unwrap();

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let game_id = game_re.captures(line).unwrap().get(1).to_i32();
        let cube_matches = cube_re.captures_iter(line).collect::<Vec<Captures>>();

        let mut is_valid_cube: bool = true;

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for cube_match in &cube_matches {
            let cube = cube_parser(&cube_match);

            match cube {
                Cube { color: CubeColor::Red, count } if count > min_red => min_red = count,
                Cube { color: CubeColor::Green, count } if count > min_green => min_green = count,
                Cube { color: CubeColor::Blue, count } if count > min_blue => min_blue = count,
                _ => {}
            }

            if is_valid_cube {
                is_valid_cube = cube.is_valid();
            }
        }

        if is_valid_cube {
            sum1 += game_id;
        }

        sum2 += min_red * min_green * min_blue;
    }

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}


fn cube_parser(match_: &Captures) -> Cube {
    return Cube {
        count: match_.get(1).to_i32(),
        color: match_.get(2).to_color(),
    };
}

pub trait IsValidCube {
    fn is_valid(&self) -> bool;
}

impl IsValidCube for Cube {
    fn is_valid(&self) -> bool {
        return match self {
            Cube { color: CubeColor::Red, count } => count <= &12,
            Cube { color: CubeColor::Green, count } => count <= &13,
            Cube { color: CubeColor::Blue, count } => count <= &14,
        };
    }
}

#[derive(Debug)]
pub struct Cube {
    count: i32,
    color: CubeColor,
}

#[derive(Debug, PartialEq)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}
