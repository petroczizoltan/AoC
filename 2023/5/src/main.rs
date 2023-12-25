use std::{env, fs::read_to_string};

#[derive(Debug)]
pub struct SrcToDstMap {
    dst_start: i64,
    src_start: i64,
    count: i64,
}

impl SrcToDstMap {
    fn in_src_range(&self, value: i64) -> bool {
        return value >= self.src_start && value < (self.src_start + self.count);
    }

    pub fn find_dst(&self, value: i64) -> Option<i64> {
       if self.in_src_range(value) {
           return Some(self.dst_start + (value - self.src_start));
       }

       return None;
    }

    pub fn from_line(line: &str) -> Self {
       let [dst, src, count] = line.split(' ').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>()[..] else { panic!() };

       return SrcToDstMap { dst_start: dst, src_start: src, count };
    }
}

#[derive(Debug)]
pub enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl MapType {
    fn next(&self) -> MapType {
        return match self {
            MapType::SeedToSoil => MapType::SoilToFertilizer,
            MapType::SoilToFertilizer => MapType::FertilizerToWater,
            MapType::FertilizerToWater => MapType::WaterToLight,
            MapType::WaterToLight => MapType::LightToTemperature,
            MapType::LightToTemperature => MapType::TemperatureToHumidity,
            MapType::TemperatureToHumidity => MapType::HumidityToLocation,
            MapType::HumidityToLocation => panic!(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let relative_input_file_path = &args[1];

    let mut input_file_path = env::current_dir().unwrap();
    input_file_path.push(relative_input_file_path);
    let file_path = input_file_path.as_path().display().to_string();
    let read_file = read_to_string(file_path).unwrap();
    let lines = read_file.lines().collect::<Vec<&str>>();

    let mut sum1: i64 = 2 << 60;
    let mut sum2: i32 = 0;

    let mut empty_line_idx = 2 << 32;

    let mut current_map_type: Option<MapType> = None;

    let mut previous_values: Vec<i64> = Vec::new();
    let mut current_values: Vec<i64> = Vec::new();

    for seed_str in lines.first().unwrap().split(' ').skip(1) {
        previous_values.push(seed_str.parse::<i64>().unwrap());
    }

    let mut map_list: Vec<SrcToDstMap> = Vec::new();

    for (index, line) in lines.iter().skip(1).enumerate() {
        if line.is_empty() {
            empty_line_idx = index;

            if current_map_type.is_some() {
                current_values.clear();
                for value in previous_values {
                    let mut found_value = false;
                    for map in &map_list {
                        let dst = map.find_dst(value);
                        match dst {
                            Some(new_value) => {
                                found_value = true;
                                current_values.push(new_value);
                                break;
                            }
                            _ => {},
                        }
                    }
                    if !found_value {
                        current_values.push(value);
                    }
                }
                previous_values = current_values.clone();
                map_list.clear();
            }
            continue;
        }

        if index == empty_line_idx + 1 {
            empty_line_idx = 2 << 32;
            current_map_type = match current_map_type {
                None => Some(MapType::SeedToSoil),
                Some(map_type) => Some(map_type.next()),
            };
            continue;
        }

        map_list.push(SrcToDstMap::from_line(line));
    }

    for val in current_values {
        if val < sum1 {
            sum1 = val;
        }
    }

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}
