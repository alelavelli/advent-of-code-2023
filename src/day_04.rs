use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub fn solve() {
    let mut file = File::open("inputs/day_04.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut points = 0;

    for line in lines {
        let numbers = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>();
        let winning_numbers: HashSet<i32> = HashSet::from_iter(
            numbers[0]
                .trim()
                .split(" ")
                .map(|x| x.parse::<i32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap()),
        );
        let extracted_numbers: HashSet<i32> = HashSet::from_iter(
            numbers[1]
                .trim()
                .split(" ")
                .map(|x| x.parse::<i32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap()),
        );

        let matching = winning_numbers
            .intersection(&extracted_numbers)
            .into_iter()
            .collect::<Vec<&i32>>();
        if matching.len() > 0 {
            if matching.len() == 1 {
                points += 1;
            } else {
                points += 2_i32.pow((matching.len() - 1) as u32);
            }
        }
        /*println!("Winning {:?}", winning_numbers);
        println!("Extracted {:?}", extracted_numbers);
        println!("Matching {:?}", matching);*/
    }
    println!("points {:?}", points);
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_04.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut cards: HashMap<usize, i32> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        *cards.entry(i + 1).or_insert(0) += 1;
        let numbers = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>();
        let winning_numbers: HashSet<i32> = HashSet::from_iter(
            numbers[0]
                .trim()
                .split(" ")
                .map(|x| x.parse::<i32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap()),
        );
        let extracted_numbers: HashSet<i32> = HashSet::from_iter(
            numbers[1]
                .trim()
                .split(" ")
                .map(|x| x.parse::<i32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap()),
        );

        let matching = winning_numbers
            .intersection(&extracted_numbers)
            .into_iter()
            .collect::<Vec<&i32>>();
        if matching.len() > 0 {
            for card_id in i + 2..(i + 2 + matching.len()) {
                *cards.entry(card_id).or_insert(0) += 1 * cards.get(&(i + 1)).unwrap();
            }
        }
    }
    println!("sum is {:?}", cards.values().sum::<i32>());
}
