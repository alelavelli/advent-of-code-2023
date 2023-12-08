use std::{fs::File, io::Read, collections::HashMap};

use num::Integer;

pub fn solve() {
    let mut file = File::open("inputs/day_08.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let instructions = *lines.get(0).unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.iter().skip(2) {
        let line_split = line.split(" = ").collect::<Vec<&str>>();
        let key = String::from(*line_split.get(0).unwrap());
        let clean_entry = line_split.get(1).unwrap().replace("(", "").replace(")", "").replace(",", "");
        let entries = clean_entry.split_whitespace().map(String::from).collect::<Vec<String>>();
        //println!("key is {key}, entries are {:?}", entries);
        map.insert(key, (entries[0].clone(), entries[1].clone()));
    }
    println!("Map: {:#?}", map);
    let mut current = String::from("AAA");
    let mut steps = 0;
    let mut instruction_iterator = instructions.chars().cycle();
    while current != "ZZZ" {
        let instruction = instruction_iterator.next().unwrap();
        steps += 1;
        current = match instruction {
            'R' => map.get(&current).unwrap().1.clone(),
            'L' => map.get(&current).unwrap().0.clone(),
            _ => panic!()
        }
    }
    println!("Steps are {steps}");
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_08.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let instructions = *lines.get(0).unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.iter().skip(2) {
        let line_split = line.split(" = ").collect::<Vec<&str>>();
        let key = String::from(*line_split.get(0).unwrap());
        let clean_entry = line_split.get(1).unwrap().replace("(", "").replace(")", "").replace(",", "");
        let entries = clean_entry.split_whitespace().map(String::from).collect::<Vec<String>>();
        //println!("key is {key}, entries are {:?}", entries);
        map.insert(key, (entries[0].clone(), entries[1].clone()));
    }
    //println!("Map: {:#?}", map);

    let starting_entries = map.keys().filter(|x| x.ends_with('A')).collect::<Vec<&String>>();
    //println!("Starting entries are {:?}", starting_entries);

    let mut steps_vector: Vec<i32> = Vec::new();
    

    for starting_entry in starting_entries {
        let mut current = String::from(starting_entry);
        let mut steps = 0;
        let mut instruction_iterator = instructions.chars().cycle();
        while !current.ends_with('Z') {
            let instruction = instruction_iterator.next().unwrap();
            steps += 1;
            current = match instruction {
                'R' => map.get(&current).unwrap().1.clone(),
                'L' => map.get(&current).unwrap().0.clone(),
                _ => panic!()
            }
        }
        steps_vector.push(steps);
    }
    println!("Steps are {:?}", steps_vector);
    let mut lcm_result: i128 = steps_vector.get(0).unwrap().lcm(steps_vector.get(1).unwrap()) as i128;
    for s in steps_vector.iter().skip(2) {
        lcm_result = lcm_result.lcm(&(*s as i128));
    }
    println!("lcm {}", lcm_result);
    
}
