use std::{env, fs::read_to_string, collections::{HashMap, BTreeMap}};

#[derive(Debug, Clone)]
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
    fn update_from_value(&self, value: u8, is_joker: bool) -> Self {
        match value {
            5 => return HandType::FiveOfAKind,
            4 if is_joker => return HandType::FiveOfAKind,
            4 => return HandType::FourOfAKind,
            3 => {
                match self {
                    HandType::HighCard if is_joker => return HandType::FourOfAKind,
                    HandType::HighCard => return HandType::ThreeOfAKind,
                    HandType::OnePair if is_joker => return HandType::FiveOfAKind,
                    HandType::OnePair => return HandType::FullHouse,
                    _ => panic!(),
                };
            },
            2 => {
                match self {
                    HandType::HighCard if is_joker => return HandType::ThreeOfAKind,
                    HandType::HighCard => return HandType::OnePair,
                    HandType::OnePair if is_joker => return HandType::FourOfAKind,
                    HandType::OnePair => return HandType::TwoPair,
                    HandType::ThreeOfAKind if is_joker => return HandType::FiveOfAKind,
                    HandType::ThreeOfAKind => return HandType::FullHouse,
                    _ => panic!(),
                };
            },
            1 if is_joker => {
                match self {
                    HandType::HighCard => return HandType::OnePair,
                    HandType::OnePair => return HandType::ThreeOfAKind,
                    HandType::TwoPair => return HandType::FullHouse,
                    HandType::ThreeOfAKind => return HandType::FourOfAKind,
                    HandType::FourOfAKind => return HandType::FiveOfAKind,
                    _ => panic!(),
                }
            }
            0 | 1 => return self.clone(),
            _ => panic!(),
        }
    }

    pub fn from_values1(values: Vec<&u8>) -> Self {
        let mut hand_type = HandType::HighCard;

        for &value in values {
            hand_type = hand_type.update_from_value(value, false);
        }

        return hand_type;
    }

    pub fn from_values2(map: &HashMap<char, u8>) -> Self {
        let mut hand_type = HandType::HighCard;

        let mut joker_count: u8 = 0;

        for (&card, &value) in map.iter() {
            if card == 'J' {
                joker_count += value as u8;
                continue;
            }

            hand_type = hand_type.update_from_value(value, false);
        }

        hand_type = hand_type.update_from_value(joker_count, true);

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
    fn to_hex1(&self) -> i64;
    fn to_hex2(&self) -> i64;
}

impl ToHex for char {
    fn to_hex1(&self) -> i64 {
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

    fn to_hex2(&self) -> i64 {
        return match self {
            'A' => 0xE,
            'K' => 0xD,
            'Q' => 0xC,
            'T' => 0xA,
            '9' => 0x9,
            '8' => 0x8,
            '7' => 0x7,
            '6' => 0x6,
            '5' => 0x5,
            '4' => 0x4,
            '3' => 0x3,
            '2' => 0x2,
            'J' => 0x1,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    hand_type1: HandType,
    hand_type2: HandType,
    cards1: i64,
    cards2: i64,
    bet: i64,
}

impl Hand {
    pub fn from_line(line: &str) -> Self {
        let [hand, bet_str] = line.split(' ').collect::<Vec<&str>>()[..] else { panic!() };

        let bet = bet_str.parse::<i64>().unwrap();

        let mut hand_map: HashMap<char, u8> = HashMap::new();

        let mut cards1: i64 = 0;
        let mut cards2: i64 = 0;

        for card in hand.chars() {
            match hand_map.insert(card, 1) {
                Some(value) => *hand_map.get_mut(&card).unwrap() = value + 1,
                None => {},
            };

            cards1 += card.to_hex1();
            cards1 *= 0xF;

            cards2 += card.to_hex2();
            cards2 *= 0xF;
        }

        let hand_type1 = HandType::from_values1(hand_map.values().collect());
        let hand_type2 = HandType::from_values2(&hand_map);

        return Hand { hand_type1, hand_type2, cards1, cards2, bet };
    }

    pub fn hand_value1(&self) -> i64 {
        return self.hand_type1.weight() * 10_i64.pow(10) + self.cards1;
    }

    pub fn hand_value2(&self) -> i64 {
        return self.hand_type2.weight() * 10_i64.pow(10) + self.cards2;
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

    let mut hands1: BTreeMap<i64, Hand> = BTreeMap::new();
    let mut hands2: BTreeMap<i64, Hand> = BTreeMap::new();

    for line in lines.iter() {
        let hand = Hand::from_line(line);
        hands1.insert(hand.hand_value1(), hand.clone());
        hands2.insert(hand.hand_value2(), hand.clone());
    }


    for (index, hand) in hands1.values().enumerate() {
        sum1 += (index as i64 + 1) * hand.bet;
    }
    for (index, hand) in hands2.values().enumerate() {
        sum2 += (index as i64 + 1) * hand.bet;
    }

    println!("1: {}", sum1);
    println!("2: {}", sum2);
}
