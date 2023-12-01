use std::{fs::File, io::Read, collections::HashMap};

pub fn solve() {
    let mut file = File::open("inputs/day_01.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    
    let mut result: Vec<i32> = Vec::new();

    for line in lines {
        let mut pair: String = String::from("");
        for c in line.chars() {
            if c.to_digit(10).is_some() {
                if pair.len() == 2 {
                    pair.remove(1);
                }
                pair.push(c);
            }
            
        }
        if pair.len() == 1 {
            pair.push(pair.chars().next().unwrap());
        }
        result.push(pair.parse::<i32>().unwrap());
    }
    let sum: i32 = result.iter().sum();
    println!("{:?}", sum);
}

pub fn solve_p2() {
    let text_digits = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];
    let digits = [
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9"
    ];
    let mut file = File::open("inputs/day_01_pt2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut result: Vec<i32> = Vec::new();
    let lines: Vec<&str> = contents.lines().collect();
    
    for line in lines {
        let mut text_indexes: Vec<usize> = Vec::new();
        let mut text_values: HashMap<usize, String> = HashMap::new();
        println!("{:?}", line.match_indices("eight").map(|x| x.0).collect::<Vec<_>>());
        for (i, t) in text_digits.iter().enumerate() {
            for index in line.match_indices(t).map(|x| x.0).collect::<Vec<_>>() {
                text_indexes.push(index);
                text_values.insert(index, i.to_string());
            }
            /*if let Some(index) = line.find(t) {
                text_indexes.push(index);
                text_values.insert(index, i.to_string());
            }*/
        }

        let mut min_index: Option<&usize> = None;
        let mut max_index: Option<&usize> = None;
        let mut first_num: Option<&str> = None;
        let mut last_num: Option<&str> = None;

        if text_indexes.len() > 0 {
            min_index = Some(text_indexes.iter().min().unwrap());
            max_index = Some(text_indexes.iter().max().unwrap());
            first_num = Some(text_values.get(min_index.as_ref().unwrap()).unwrap());
            last_num = Some(text_values.get(max_index.as_ref().unwrap()).unwrap());
        }

        let mut digit_indexes: Vec<usize> = Vec::new();
        let mut digit_values: HashMap<usize, String> = HashMap::new();
        for (i, d) in digits.iter().enumerate() {
            for index in line.match_indices(d).map(|x| x.0).collect::<Vec<_>>() {
                digit_indexes.push(index);
                digit_values.insert(index, i.to_string());
            }
            /*if let Some(index) = line.find(d) {
                digit_indexes.push(index);
                digit_values.insert(index, i.to_string());
            }*/
        }
        
        println!("line {:?}", line);
        println!("text indexes {:?}", text_indexes);
        println!("text values {:?}", text_values);
        println!("digit indexes {:?}", digit_indexes);
        println!("digit values {:?}", digit_values);
        
        if digit_indexes.len() > 0 {
            if !min_index.is_some_and(|x| x <= digit_indexes.iter().min().unwrap())  {
                min_index = Some(digit_indexes.iter().min().unwrap());
                first_num = Some(digit_values.get(min_index.as_ref().unwrap()).unwrap());
            }
            if ! max_index.is_some_and(|x| x >= digit_indexes.iter().max().unwrap())  {
                max_index = Some(digit_indexes.iter().max().unwrap());
                last_num = Some(digit_values.get(max_index.as_ref().unwrap()).unwrap());
            }
        }
        let mut num: String = String::new();
        num.push_str(first_num.unwrap());
        num.push_str(last_num.unwrap());
        result.push(num.parse::<i32>().unwrap());
        println!("num {:?}", num);
        println!();
    }
    
    let sum: i32 = result.iter().sum();
    println!("{:?}", sum);
}