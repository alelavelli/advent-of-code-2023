use std::{fs::File, io::Read, collections::HashMap};

fn hand_type(hand: &Hand) -> u32 {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for card in hand.cards.iter() {
        *map.entry(*card).or_insert(0) += 1;
    }
    
    match map.len() {
        // five of a kind
        1 => 7,
        2 => {
            // four of a kind or full house
            //if **set.iter().max().unwrap() == 4 {
            if map.values().max().unwrap() == &4 {
                6
            } else {
                5
            }
        },
        3 => {
            // three of a kind or two pair
            if map.values().max().unwrap() == &3 {
                4
            } else {
                3
            }

        },
        // one pair
        4 => 2,
        // high card
        _ => 1
    }

}

fn hand_type_jocker(hand: &Hand) -> u32 {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for card in hand.cards.iter() {
        *map.entry(*card).or_insert(0) += 1;
    }
    if map.get(&0).is_some() {
        match map.len() - 1 {
            0 => 7,
            1 => 7,
            2 => {
                if 
                (map.get(&0).unwrap() == &1) &
                (map.iter().filter(|(k, v)| *k != &0).map(|(k, v)| v).max().unwrap() == &2) {
                    5
                } else {
                    6
                }

                /*if 
                    (map.values().max().unwrap() == &3) 
                    | (map.values().max().unwrap() == &2)
                 {
                    6
                } else {
                    5
                }*/

            },
            3 => 4,
            // one pair
            4 => 2,
            // high card
            _ => 1
        }
    } else 
    {
        hand_type(hand)
    }
}


#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    jocker: bool
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_fn = if self.jocker {
            hand_type_jocker
        } else {
            hand_type
        };
        if type_fn(self) > type_fn(other) {
            std::cmp::Ordering::Greater
        } else if type_fn(self) < type_fn(other) {
            std::cmp::Ordering::Less
        } else {
            for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
                if self_card > other_card {
                    return std::cmp::Ordering::Greater;
                } else if self_card < other_card {
                    return std::cmp::Ordering::Less;
                } else {

                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

pub fn solve() {
    let mut file = File::open("inputs/day_07.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let mut cards: Vec<u32> = Vec::new();
        for card in split.get(0).unwrap().chars() {
            let parsed_card = match card {
                // A,   K,  Q,  J,  T
                //14, 13, 12, 11, 10
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                x => x.to_digit(10).unwrap()
            };
            cards.push(parsed_card);
        }
        let bid = split.get(1).unwrap().parse::<u32>().unwrap();
        let hand = Hand {cards, bid, jocker: false};
        //println!("Hand {:?} with type {}", hand, hand_type(&hand));
        hands.push(hand);
    }
    hands.sort();
    //println!("hands are {:#?}", hands);
    let mut total_win = 0;
    for (i, hand) in hands.iter().enumerate() {
        total_win += (i+1) as u32 * hand.bid;
    }
    println!("Total win is {total_win}");
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_07.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let mut cards: Vec<u32> = Vec::new();
        for card in split.get(0).unwrap().chars() {
            let parsed_card = match card {
                // A,   K,  Q,  J,  T
                //14, 13, 12, 11, 10
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                x => x.to_digit(10).unwrap()
            };
            cards.push(parsed_card);
        }
        let bid = split.get(1).unwrap().parse::<u32>().unwrap();
        let hand = Hand {cards, bid, jocker: true};
        //println!("Hand {:?} with type {}", hand, hand_type_jocker(&hand));
        hands.push(hand);
    }
    hands.sort();
    //println!("hands are {:#?}", hands);
    let mut total_win = 0;
    for (i, hand) in hands.iter().enumerate() {
        total_win += (i+1) as u32 * hand.bid;
    }

    println!("Total win is {total_win}");
}