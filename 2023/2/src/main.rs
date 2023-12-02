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

    let mut sum: i32 = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let game_id = game_re.captures(line).unwrap().get(1).to_i32();
        let cube_matches = cube_re.captures_iter(line).collect::<Vec<Captures>>();

        let mut is_valid_cube: bool = true;

        for cube_match in &cube_matches {
            let cube = cube_parser(&cube_match);

            is_valid_cube = cube.is_valid();

            if !is_valid_cube {
                break;
            }
        }

        if is_valid_cube {
            println!("{}", game_id);
            sum += game_id;
        }
    }

    println!("{}", sum);
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
        return self.color == CubeColor::Red && self.count <= 12 ||
                self.color == CubeColor::Green && self.count <= 13 ||
                self.color == CubeColor::Blue && self.count <= 14;
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
