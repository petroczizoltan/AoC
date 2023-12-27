use std::{env, fs::read_to_string, collections::{HashMap, BTreeMap}};

#[derive(Debug)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn from_values(values: Vec<&u8>) -> Self {
        let mut hand_type = HandType::HighCard;

        for value in values {
            match value {
                5 => return HandType::FiveOfAKind,
                4 => return HandType::FourOfAKind,
                3 => {
                    match hand_type {
                        HandType::HighCard => hand_type = HandType::ThreeOfAKind,
                        HandType::OnePair => hand_type = HandType::FullHouse,
                        _ => panic!(),
                    };
                },
                2 => {
                    match hand_type {
                        HandType::HighCard => hand_type = HandType::OnePair,
                        HandType::OnePair => hand_type = HandType::TwoPair,
                        HandType::ThreeOfAKind => hand_type = HandType::FullHouse,
                        _ => panic!(),
                    };
                },
                1 => {}
                _ => panic!(),
            }
        }

        return hand_type;
    }

    pub fn weight(&self) -> i64 {
        return match self {
            HandType::HighCard => 0,
            HandType::OnePair => 1,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 5,
            HandType::FiveOfAKind => 6,
        };
    }
}

trait ToHex {
    fn to_hex(&self) -> i64;
}

impl ToHex for char {
    fn to_hex(&self) -> i64 {
        return match self {
            'A' => 0xE,
            'K' => 0xD,
            'Q' => 0xC,
            'J' => 0xB,
            'T' => 0xA,
            '9' => 0x9,
            '8' => 0x8,
            '7' => 0x7,
            '6' => 0x6,
            '5' => 0x5,
            '4' => 0x4,
            '3' => 0x3,
            '2' => 0x2,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    hand_type: HandType,
    cards: i64,
    bet: i64,
}

impl Hand {
    pub fn from_line(line: &str) -> Self {
        let [hand, bet_str] = line.split(' ').collect::<Vec<&str>>()[..] else { panic!() };

        let bet = bet_str.parse::<i64>().unwrap();

        let mut hand_map: HashMap<char, u8> = HashMap::new();

        let mut cards: i64 = 0;

        for card in hand.chars() {
            match hand_map.insert(card, 1) {
                Some(value) => *hand_map.get_mut(&card).unwrap() = value + 1,
                None => {},
            };

            cards += card.to_hex();
            cards *= 0xF;
        }

        let hand_type = HandType::from_values(hand_map.values().collect());

        return Hand { hand_type, cards, bet };
    }

    pub fn hand_value(&self) -> i64 {
        return self.hand_type.weight() * 10_i64.pow(10) + self.cards;
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

    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;

    let mut hands: BTreeMap<i64, Hand> = BTreeMap::new();

    for line in lines.iter() {
        let hand = Hand::from_line(line);
        hands.insert(hand.hand_value(), hand);
    }


    for (index, hand) in hands.values().enumerate() {
        sum1 += (index as i64 + 1) * hand.bet;
    }

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}
